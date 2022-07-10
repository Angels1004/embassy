#![no_std]
#![no_main]
#![feature(type_alias_impl_trait, concat_bytes)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

use core::slice;

use embassy::time::{block_for, Duration, Timer};
use embassy::util::yield_now;
use embassy_rp::gpio::{Flex, Output, Pin};

fn swap16(x: u32) -> u32 {
    (x & 0xFF00FF00) >> 8 | (x & 0x00FF00FF) << 8
}

fn cmd_word(write: bool, incr: bool, func: u32, addr: u32, len: u32) -> u32 {
    (write as u32) << 31 | (incr as u32) << 30 | (func & 0b11) << 28 | (addr & 0x1FFFF) << 11 | (len & 0x7FF)
}

const FUNC_BUS: u32 = 0;
const FUNC_BACKPLANE: u32 = 1;
const FUNC_WLAN: u32 = 2;
const FUNC_BT: u32 = 3;

const REG_BUS_CTRL: u32 = 0x0;
const REG_BUS_INTERRUPT: u32 = 0x04; // 16 bits - Interrupt status
const REG_BUS_INTERRUPT_ENABLE: u32 = 0x06; // 16 bits - Interrupt mask
const REG_BUS_STATUS: u32 = 0x8;
const REG_BUS_FEEDBEAD: u32 = 0x14;
const REG_BUS_TEST: u32 = 0x18;
const REG_BUS_RESP_DELAY: u32 = 0x1c;

// SPI_STATUS_REGISTER bits
const STATUS_DATA_NOT_AVAILABLE: u32 = 0x00000001;
const STATUS_UNDERFLOW: u32 = 0x00000002;
const STATUS_OVERFLOW: u32 = 0x00000004;
const STATUS_F2_INTR: u32 = 0x00000008;
const STATUS_F3_INTR: u32 = 0x00000010;
const STATUS_F2_RX_READY: u32 = 0x00000020;
const STATUS_F3_RX_READY: u32 = 0x00000040;
const STATUS_HOST_CMD_DATA_ERR: u32 = 0x00000080;
const STATUS_F2_PKT_AVAILABLE: u32 = 0x00000100;
const STATUS_F2_PKT_LEN_MASK: u32 = 0x000FFE00;
const STATUS_F2_PKT_LEN_SHIFT: u32 = 9;
const STATUS_F3_PKT_AVAILABLE: u32 = 0x00100000;
const STATUS_F3_PKT_LEN_MASK: u32 = 0xFFE00000;
const STATUS_F3_PKT_LEN_SHIFT: u32 = 21;

const REG_BACKPLANE_GPIO_SELECT: u32 = 0x10005;
const REG_BACKPLANE_GPIO_OUTPUT: u32 = 0x10006;
const REG_BACKPLANE_GPIO_ENABLE: u32 = 0x10007;
const REG_BACKPLANE_FUNCTION2_WATERMARK: u32 = 0x10008;
const REG_BACKPLANE_DEVICE_CONTROL: u32 = 0x10009;
const REG_BACKPLANE_BACKPLANE_ADDRESS_LOW: u32 = 0x1000A;
const REG_BACKPLANE_BACKPLANE_ADDRESS_MID: u32 = 0x1000B;
const REG_BACKPLANE_BACKPLANE_ADDRESS_HIGH: u32 = 0x1000C;
const REG_BACKPLANE_FRAME_CONTROL: u32 = 0x1000D;
const REG_BACKPLANE_CHIP_CLOCK_CSR: u32 = 0x1000E;
const REG_BACKPLANE_PULL_UP: u32 = 0x1000F;
const REG_BACKPLANE_READ_FRAME_BC_LOW: u32 = 0x1001B;
const REG_BACKPLANE_READ_FRAME_BC_HIGH: u32 = 0x1001C;
const REG_BACKPLANE_WAKEUP_CTRL: u32 = 0x1001E;
const REG_BACKPLANE_SLEEP_CSR: u32 = 0x1001F;

const BACKPLANE_WINDOW_SIZE: usize = 0x8000;
const BACKPLANE_ADDRESS_MASK: u32 = 0x7FFF;
const BACKPLANE_ADDRESS_32BIT_FLAG: u32 = 0x08000;
const BACKPLANE_MAX_TRANSFER_SIZE: usize = 64;

const AI_IOCTRL_OFFSET: u32 = 0x408;
const AI_IOCTRL_BIT_FGC: u8 = 0x0002;
const AI_IOCTRL_BIT_CLOCK_EN: u8 = 0x0001;
const AI_IOCTRL_BIT_CPUHALT: u8 = 0x0020;

const AI_RESETCTRL_OFFSET: u32 = 0x800;
const AI_RESETCTRL_BIT_RESET: u8 = 1;

const AI_RESETSTATUS_OFFSET: u32 = 0x804;

const TEST_PATTERN: u32 = 0x12345678;
const FEEDBEAD: u32 = 0xFEEDBEAD;

// SPI_INTERRUPT_REGISTER and SPI_INTERRUPT_ENABLE_REGISTER Bits
const IRQ_DATA_UNAVAILABLE: u16 = 0x0001; // Requested data not available; Clear by writing a "1"
const IRQ_F2_F3_FIFO_RD_UNDERFLOW: u16 = 0x0002;
const IRQ_F2_F3_FIFO_WR_OVERFLOW: u16 = 0x0004;
const IRQ_COMMAND_ERROR: u16 = 0x0008; // Cleared by writing 1
const IRQ_DATA_ERROR: u16 = 0x0010; // Cleared by writing 1
const IRQ_F2_PACKET_AVAILABLE: u16 = 0x0020;
const IRQ_F3_PACKET_AVAILABLE: u16 = 0x0040;
const IRQ_F1_OVERFLOW: u16 = 0x0080; // Due to last write. Bkplane has pending write requests
const IRQ_MISC_INTR0: u16 = 0x0100;
const IRQ_MISC_INTR1: u16 = 0x0200;
const IRQ_MISC_INTR2: u16 = 0x0400;
const IRQ_MISC_INTR3: u16 = 0x0800;
const IRQ_MISC_INTR4: u16 = 0x1000;
const IRQ_F1_INTR: u16 = 0x2000;
const IRQ_F2_INTR: u16 = 0x4000;
const IRQ_F3_INTR: u16 = 0x8000;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Core {
    WLAN = 0,
    SOCSRAM = 1,
    SDIOD = 2,
}

impl Core {
    fn base_addr(&self) -> u32 {
        match self {
            Self::WLAN => CHIP.arm_core_base_address,
            Self::SOCSRAM => CHIP.socsram_wrapper_base_address,
            Self::SDIOD => CHIP.sdiod_core_base_address,
        }
    }
}

struct Chip {
    arm_core_base_address: u32,
    socsram_base_address: u32,
    socsram_wrapper_base_address: u32,
    sdiod_core_base_address: u32,
    pmu_base_address: u32,
    chip_ram_size: u32,
    atcm_ram_base_address: u32,
    socram_srmem_size: u32,
    chanspec_band_mask: u32,
    chanspec_band_2g: u32,
    chanspec_band_5g: u32,
    chanspec_band_shift: u32,
    chanspec_bw_10: u32,
    chanspec_bw_20: u32,
    chanspec_bw_40: u32,
    chanspec_bw_mask: u32,
    chanspec_bw_shift: u32,
    chanspec_ctl_sb_lower: u32,
    chanspec_ctl_sb_upper: u32,
    chanspec_ctl_sb_none: u32,
    chanspec_ctl_sb_mask: u32,
}

const WRAPPER_REGISTER_OFFSET: u32 = 0x100000;

// Data for CYW43439
const CHIP: Chip = Chip {
    arm_core_base_address: 0x18003000 + WRAPPER_REGISTER_OFFSET,
    socsram_base_address: 0x18004000,
    socsram_wrapper_base_address: 0x18004000 + WRAPPER_REGISTER_OFFSET,
    sdiod_core_base_address: 0x18002000,
    pmu_base_address: 0x18000000,
    chip_ram_size: 512 * 1024,
    atcm_ram_base_address: 0,
    socram_srmem_size: 64 * 1024,
    chanspec_band_mask: 0xc000,
    chanspec_band_2g: 0x0000,
    chanspec_band_5g: 0xc000,
    chanspec_band_shift: 14,
    chanspec_bw_10: 0x0800,
    chanspec_bw_20: 0x1000,
    chanspec_bw_40: 0x1800,
    chanspec_bw_mask: 0x3800,
    chanspec_bw_shift: 11,
    chanspec_ctl_sb_lower: 0x0000,
    chanspec_ctl_sb_upper: 0x0100,
    chanspec_ctl_sb_none: 0x0000,
    chanspec_ctl_sb_mask: 0x0700,
};

#[derive(Clone, Copy)]
#[repr(C)]
struct SdpcmHeader {
    len: u16,
    len_inv: u16,
    /// Rx/Tx sequence number
    sequence: u8,
    ///  4 MSB Channel number, 4 LSB arbitrary flag
    channel_and_flags: u8,
    /// Length of next data frame, reserved for Tx
    next_length: u8,
    /// Data offset
    header_length: u8,
    /// Flow control bits, reserved for Tx
    wireless_flow_control: u8,
    /// Maximum Sequence number allowed by firmware for Tx
    bus_data_credit: u8,
    /// Reserved
    reserved: [u8; 2],
}

#[derive(Clone, Copy)]
#[repr(C)]
struct CdcHeader {
    cmd: u32,
    out_len: u16,
    in_len: u16,
    flags: u16,
    id: u16,
    status: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct BdcHeader {
    flags: u8,
    /// 802.1d Priority (low 3 bits)
    priority: u8,
    flags2: u8,
    /// Offset from end of BDC header to packet data, in 4-uint8_t words. Leaves room for optional headers.
    data_offset: u8,
}

macro_rules! impl_bytes {
    ($t:ident) => {
        impl $t {
            const SIZE: usize = core::mem::size_of::<Self>();

            pub fn to_bytes(&self) -> [u8; Self::SIZE] {
                unsafe { core::mem::transmute(*self) }
            }

            pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
                unsafe { core::mem::transmute(*bytes) }
            }
        }
    };
}
impl_bytes!(SdpcmHeader);
impl_bytes!(CdcHeader);
impl_bytes!(BdcHeader);

pub struct Driver<'a, PWR: Pin, CS: Pin, CLK: Pin, DIO: Pin> {
    pwr: Output<'a, PWR>,

    /// SPI chip-select.
    cs: Output<'a, CS>,

    /// SPI clock
    clk: Output<'a, CLK>,

    /// 4 signals, all in one!!
    /// - SPI MISO
    /// - SPI MOSI
    /// - IRQ
    /// - strap to set to gSPI mode on boot.
    dio: Flex<'a, DIO>,

    backplane_window: u32,
}

impl<'a, PWR: Pin, CS: Pin, CLK: Pin, DIO: Pin> Driver<'a, PWR, CS, CLK, DIO> {
    pub fn new(pwr: Output<'a, PWR>, cs: Output<'a, CS>, clk: Output<'a, CLK>, dio: Flex<'a, DIO>) -> Self {
        Self {
            pwr,
            cs,
            clk,
            dio,
            backplane_window: 0xAAAA_AAAA,
        }
    }

    pub async fn init(&mut self) {
        // Set strap to select gSPI mode.
        self.dio.set_as_output();
        self.dio.set_low();

        // Reset
        self.pwr.set_low();
        Timer::after(Duration::from_millis(20)).await;
        self.pwr.set_high();
        Timer::after(Duration::from_millis(250)).await;

        info!("waiting for ping...");
        while self.read32_swapped(REG_BUS_FEEDBEAD) != FEEDBEAD {}
        info!("ping ok");

        self.write32_swapped(0x18, TEST_PATTERN);
        let val = self.read32_swapped(REG_BUS_TEST);
        assert_eq!(val, TEST_PATTERN);

        // 32bit, big endian.
        self.write32_swapped(REG_BUS_CTRL, 0x00010033);

        let val = self.read32(FUNC_BUS, REG_BUS_FEEDBEAD);
        assert_eq!(val, FEEDBEAD);
        let val = self.read32(FUNC_BUS, REG_BUS_TEST);
        assert_eq!(val, TEST_PATTERN);

        // No response delay in any of the funcs.
        // seems to break backplane??? eat the 4-byte delay instead, that's what the vendor drivers do...
        //self.write32(FUNC_BUS, REG_BUS_RESP_DELAY, 0);

        // Init ALP (no idea what that stands for) clock
        self.write8(FUNC_BACKPLANE, REG_BACKPLANE_CHIP_CLOCK_CSR, 0x08);
        info!("waiting for clock...");
        while self.read8(FUNC_BACKPLANE, REG_BACKPLANE_CHIP_CLOCK_CSR) & 0x40 == 0 {}
        info!("clock ok");

        let chip_id = self.bp_read16(0x1800_0000);
        info!("chip ID: {}", chip_id);

        // Upload firmware.
        self.core_disable(Core::WLAN);
        self.core_reset(Core::SOCSRAM);
        self.bp_write32(CHIP.socsram_base_address + 0x10, 3);
        self.bp_write32(CHIP.socsram_base_address + 0x44, 0);

        // I'm flashing the firmwares independently at hardcoded addresses, instead of baking them
        // into the program with `include_bytes!` or similar, so that flashing the program stays fast.
        //
        // Flash them like this, also don't forget to update the lengths below if you change them!.
        //
        // probe-rs-cli download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
        // probe-rs-cli download 43439A0.clm_blob --format bin --chip RP2040 --base-address 0x10140000
        let fw = unsafe { slice::from_raw_parts(0x10100000 as *const u8, 224190) };
        let clm = unsafe { slice::from_raw_parts(0x10140000 as *const u8, 4752) };

        let ram_addr = CHIP.atcm_ram_base_address;

        info!("loading fw");
        self.bp_write(ram_addr, fw);

        info!("verifying fw");
        let mut buf = [0; 1024];
        for (i, chunk) in fw.chunks(1024).enumerate() {
            let buf = &mut buf[..chunk.len()];
            self.bp_read(ram_addr + i as u32 * 1024, buf);
            assert_eq!(chunk, buf);
        }

        info!("loading nvram");
        // Round up to 4 bytes.
        let nvram_len = (NVRAM.len() + 3) / 4 * 4;
        self.bp_write(ram_addr + CHIP.chip_ram_size - 4 - nvram_len as u32, NVRAM);

        let nvram_len_words = nvram_len as u32 / 4;
        let nvram_len_magic = (!nvram_len_words << 16) | nvram_len_words;
        self.bp_write32(ram_addr + CHIP.chip_ram_size - 4, nvram_len_magic);

        // Start core!
        info!("starting up core...");
        self.core_reset(Core::WLAN);
        assert!(self.core_is_up(Core::WLAN));

        while self.read8(FUNC_BACKPLANE, REG_BACKPLANE_CHIP_CLOCK_CSR) & 0x80 == 0 {}

        // "Set up the interrupt mask and enable interrupts"
        self.bp_write32(CHIP.sdiod_core_base_address + 0x24, 0xF0);

        // "Lower F2 Watermark to avoid DMA Hang in F2 when SD Clock is stopped."
        // Sounds scary...
        self.write8(FUNC_BACKPLANE, REG_BACKPLANE_FUNCTION2_WATERMARK, 32);

        // wait for wifi startup
        info!("waiting for wifi init...");
        while self.read32(FUNC_BUS, REG_BUS_STATUS) & STATUS_F2_RX_READY == 0 {}

        // Some random configs related to sleep.
        // I think they're not needed if we don't want sleep...???
        /*
        let mut val = self.read8(FUNC_BACKPLANE, REG_BACKPLANE_WAKEUP_CTRL);
        val |= 0x02; // WAKE_TILL_HT_AVAIL
        self.write8(FUNC_BACKPLANE, REG_BACKPLANE_WAKEUP_CTRL, val);
        self.write8(FUNC_BUS, 0xF0, 0x08); // SDIOD_CCCR_BRCM_CARDCAP.CMD_NODEC = 1
        self.write8(FUNC_BACKPLANE, REG_BACKPLANE_CHIP_CLOCK_CSR, 0x02); // SBSDIO_FORCE_HT

        let mut val = self.read8(FUNC_BACKPLANE, REG_BACKPLANE_SLEEP_CSR);
        val |= 0x01; // SBSDIO_SLPCSR_KEEP_SDIO_ON
        self.write8(FUNC_BACKPLANE, REG_BACKPLANE_SLEEP_CSR, val);

        // clear pulls
        self.write8(FUNC_BACKPLANE, REG_BACKPLANE_PULL_UP, 0);
        let _ = self.read8(FUNC_BACKPLANE, REG_BACKPLANE_PULL_UP);
         */

        let mut buf = [0; 8 + 12 + 1024];
        buf[0..8].copy_from_slice(b"clmload\x00");
        buf[8..20].copy_from_slice(b"\x02\x10\x02\x00\x00\x04\x00\x00\x00\x00\x00\x00");
        buf[20..].copy_from_slice(&clm[..1024]);
        self.send_ioctl(2, 263, 0, &buf);

        info!("init done ");

        let mut old_irq = 0;
        let mut buf = [0; 2048];
        loop {
            let irq = self.read16(FUNC_BUS, REG_BUS_INTERRUPT);
            if irq != old_irq {
                info!("irq: {:04x}", irq);
            }
            old_irq = irq;

            if irq & IRQ_F2_PACKET_AVAILABLE != 0 {
                let mut status = 0xFFFF_FFFF;
                while status == 0xFFFF_FFFF {
                    status = self.read32(FUNC_BUS, REG_BUS_STATUS);
                }

                if status & STATUS_F2_PKT_AVAILABLE != 0 {
                    let len = (status & STATUS_F2_PKT_LEN_MASK) >> STATUS_F2_PKT_LEN_SHIFT;
                    info!("got len {}", len);

                    let cmd = cmd_word(false, true, FUNC_WLAN, 0, len);

                    self.cs.set_low();
                    self.spi_write(&cmd.to_le_bytes());
                    self.spi_read(&mut buf[..len as usize]);
                    // pad to 32bit
                    let mut junk = [0; 4];
                    if len % 4 != 0 {
                        self.spi_read(&mut junk[..(4 - len as usize % 4)]);
                    }
                    self.cs.set_high();

                    info!("rxd packet {:02x}", &buf[..len as usize]);

                    self.rx(&buf[..len as usize]);
                }
            }

            yield_now().await;
        }
    }

    fn rx(&mut self, packet: &[u8]) {
        if packet.len() < SdpcmHeader::SIZE {
            warn!("packet too short, len={}", packet.len());
            return;
        }

        let sdpcm_header = SdpcmHeader::from_bytes(packet[..SdpcmHeader::SIZE].try_into().unwrap());

        if sdpcm_header.len != !sdpcm_header.len_inv {
            warn!("len inv mismatch");
            return;
        }
        if sdpcm_header.len as usize != packet.len() {
            // TODO: is this guaranteed??
            warn!("len from header doesn't match len from spi");
            return;
        }

        let channel = sdpcm_header.channel_and_flags & 0x0f;

        match channel {
            0 => {
                if packet.len() < SdpcmHeader::SIZE + CdcHeader::SIZE {
                    warn!("control packet too short, len={}", packet.len());
                    return;
                }

                let cdc_header =
                    CdcHeader::from_bytes(packet[SdpcmHeader::SIZE..][..CdcHeader::SIZE].try_into().unwrap());

                // TODO check cdc_header.id matches
                // TODO check status
            }
            _ => {}
        }
    }

    fn send_ioctl(&mut self, kind: u32, cmd: u32, iface: u32, data: &[u8]) {
        let mut buf = [0; 2048];

        let total_len = SdpcmHeader::SIZE + CdcHeader::SIZE + data.len();

        let sdpcm_header = SdpcmHeader {
            len: total_len as u16,
            len_inv: !total_len as u16,
            sequence: 0x02,       // todo
            channel_and_flags: 0, // control channle
            next_length: 0,
            header_length: SdpcmHeader::SIZE as _,
            wireless_flow_control: 0,
            bus_data_credit: 0,
            reserved: [0, 0],
        };

        let cdc_header = CdcHeader {
            cmd: cmd,
            out_len: data.len() as _,
            in_len: 0,
            flags: kind as u16 | (iface as u16) << 12,
            id: 1, // todo
            status: 0,
        };

        buf[0..SdpcmHeader::SIZE].copy_from_slice(&sdpcm_header.to_bytes());
        buf[SdpcmHeader::SIZE..][..CdcHeader::SIZE].copy_from_slice(&cdc_header.to_bytes());
        buf[SdpcmHeader::SIZE + CdcHeader::SIZE..][..data.len()].copy_from_slice(data);

        info!("txing {:02x}", &buf[..total_len]);

        let cmd = cmd_word(true, true, FUNC_WLAN, 0, total_len as _);
        self.cs.set_low();
        self.spi_write(&cmd.to_le_bytes());
        self.spi_write(&buf[..total_len]);
        self.cs.set_high();
    }

    fn core_disable(&mut self, core: Core) {
        let base = core.base_addr();

        // Dummy read?
        let _ = self.bp_read8(base + AI_RESETCTRL_OFFSET);

        // Check it isn't already reset
        let r = self.bp_read8(base + AI_RESETCTRL_OFFSET);
        if r & AI_RESETCTRL_BIT_RESET != 0 {
            return;
        }

        self.bp_write8(base + AI_IOCTRL_OFFSET, 0);
        let _ = self.bp_read8(base + AI_IOCTRL_OFFSET);

        block_for(Duration::from_millis(1));

        self.bp_write8(base + AI_RESETCTRL_OFFSET, AI_RESETCTRL_BIT_RESET);
        let _ = self.bp_read8(base + AI_RESETCTRL_OFFSET);
    }

    fn core_reset(&mut self, core: Core) {
        self.core_disable(core);

        let base = core.base_addr();
        self.bp_write8(base + AI_IOCTRL_OFFSET, AI_IOCTRL_BIT_FGC | AI_IOCTRL_BIT_CLOCK_EN);
        let _ = self.bp_read8(base + AI_IOCTRL_OFFSET);

        self.bp_write8(base + AI_RESETCTRL_OFFSET, 0);

        block_for(Duration::from_millis(1));

        self.bp_write8(base + AI_IOCTRL_OFFSET, AI_IOCTRL_BIT_CLOCK_EN);
        let _ = self.bp_read8(base + AI_IOCTRL_OFFSET);

        block_for(Duration::from_millis(1));
    }

    fn core_is_up(&mut self, core: Core) -> bool {
        let base = core.base_addr();

        let io = self.bp_read8(base + AI_IOCTRL_OFFSET);
        if io & (AI_IOCTRL_BIT_FGC | AI_IOCTRL_BIT_CLOCK_EN) != AI_IOCTRL_BIT_CLOCK_EN {
            debug!("core_is_up: returning false due to bad ioctrl {:02x}", io);
            return false;
        }

        let r = self.bp_read8(base + AI_RESETCTRL_OFFSET);
        if r & (AI_RESETCTRL_BIT_RESET) != 0 {
            debug!("core_is_up: returning false due to bad resetctrl {:02x}", r);
            return false;
        }

        true
    }

    fn bp_read(&mut self, mut addr: u32, mut data: &mut [u8]) {
        // It seems the HW force-aligns the addr
        // to 2 if data.len() >= 2
        // to 4 if data.len() >= 4
        // To simplify, enforce 4-align for now.
        assert!(addr % 4 == 0);

        while !data.is_empty() {
            // Ensure transfer doesn't cross a window boundary.
            let window_offs = addr & BACKPLANE_ADDRESS_MASK;
            let window_remaining = BACKPLANE_WINDOW_SIZE - window_offs as usize;

            let len = data.len().min(BACKPLANE_MAX_TRANSFER_SIZE).min(window_remaining);

            self.backplane_set_window(addr);

            let cmd = cmd_word(false, true, FUNC_BACKPLANE, window_offs, len as u32);
            self.cs.set_low();
            self.spi_write(&cmd.to_le_bytes());

            // 4-byte response delay.
            let mut junk = [0; 4];
            self.spi_read(&mut junk);

            // Read data
            self.spi_read(&mut data[..len]);

            // pad to 32bit
            if len % 4 != 0 {
                self.spi_read(&mut junk[..(4 - len % 4)]);
            }
            self.cs.set_high();

            // Advance ptr.
            addr += len as u32;
            data = &mut data[len..];
        }
    }

    fn bp_write(&mut self, mut addr: u32, mut data: &[u8]) {
        // It seems the HW force-aligns the addr
        // to 2 if data.len() >= 2
        // to 4 if data.len() >= 4
        // To simplify, enforce 4-align for now.
        assert!(addr % 4 == 0);

        while !data.is_empty() {
            // Ensure transfer doesn't cross a window boundary.
            let window_offs = addr & BACKPLANE_ADDRESS_MASK;
            let window_remaining = BACKPLANE_WINDOW_SIZE - window_offs as usize;

            let len = data.len().min(BACKPLANE_MAX_TRANSFER_SIZE).min(window_remaining);

            self.backplane_set_window(addr);

            let cmd = cmd_word(true, true, FUNC_BACKPLANE, window_offs, len as u32);
            self.cs.set_low();
            self.spi_write(&cmd.to_le_bytes());
            self.spi_write(&data[..len]);
            // pad to 32bit
            if len % 4 != 0 {
                let zeros = [0; 4];
                self.spi_write(&zeros[..(4 - len % 4)]);
            }
            self.cs.set_high();

            // Advance ptr.
            addr += len as u32;
            data = &data[len..];
        }
    }

    fn bp_read8(&mut self, addr: u32) -> u8 {
        self.backplane_readn(addr, 1) as u8
    }

    fn bp_write8(&mut self, addr: u32, val: u8) {
        self.backplane_writen(addr, val as u32, 1)
    }

    fn bp_read16(&mut self, addr: u32) -> u16 {
        self.backplane_readn(addr, 2) as u16
    }

    fn bp_write16(&mut self, addr: u32, val: u16) {
        self.backplane_writen(addr, val as u32, 2)
    }

    fn bp_read32(&mut self, addr: u32) -> u32 {
        self.backplane_readn(addr, 4)
    }

    fn bp_write32(&mut self, addr: u32, val: u32) {
        self.backplane_writen(addr, val, 4)
    }

    fn backplane_readn(&mut self, addr: u32, len: u32) -> u32 {
        self.backplane_set_window(addr);

        let mut bus_addr = addr & BACKPLANE_ADDRESS_MASK;
        if len == 4 {
            bus_addr |= BACKPLANE_ADDRESS_32BIT_FLAG
        }
        self.readn(FUNC_BACKPLANE, bus_addr, len)
    }

    fn backplane_writen(&mut self, addr: u32, val: u32, len: u32) {
        self.backplane_set_window(addr);

        let mut bus_addr = addr & BACKPLANE_ADDRESS_MASK;
        if len == 4 {
            bus_addr |= BACKPLANE_ADDRESS_32BIT_FLAG
        }
        self.writen(FUNC_BACKPLANE, bus_addr, val, len)
    }

    fn backplane_set_window(&mut self, addr: u32) {
        let new_window = addr & !BACKPLANE_ADDRESS_MASK;

        if (new_window >> 24) as u8 != (self.backplane_window >> 24) as u8 {
            self.write8(
                FUNC_BACKPLANE,
                REG_BACKPLANE_BACKPLANE_ADDRESS_HIGH,
                (new_window >> 24) as u8,
            );
        }
        if (new_window >> 16) as u8 != (self.backplane_window >> 16) as u8 {
            self.write8(
                FUNC_BACKPLANE,
                REG_BACKPLANE_BACKPLANE_ADDRESS_MID,
                (new_window >> 16) as u8,
            );
        }
        if (new_window >> 8) as u8 != (self.backplane_window >> 8) as u8 {
            self.write8(
                FUNC_BACKPLANE,
                REG_BACKPLANE_BACKPLANE_ADDRESS_LOW,
                (new_window >> 8) as u8,
            );
        }
        self.backplane_window = new_window;
    }

    fn read8(&mut self, func: u32, addr: u32) -> u8 {
        self.readn(func, addr, 1) as u8
    }

    fn write8(&mut self, func: u32, addr: u32, val: u8) {
        self.writen(func, addr, val as u32, 1)
    }

    fn read16(&mut self, func: u32, addr: u32) -> u16 {
        self.readn(func, addr, 2) as u16
    }

    fn write16(&mut self, func: u32, addr: u32, val: u16) {
        self.writen(func, addr, val as u32, 2)
    }

    fn read32(&mut self, func: u32, addr: u32) -> u32 {
        self.readn(func, addr, 4)
    }

    fn write32(&mut self, func: u32, addr: u32, val: u32) {
        self.writen(func, addr, val, 4)
    }

    fn readn(&mut self, func: u32, addr: u32, len: u32) -> u32 {
        let cmd = cmd_word(false, true, func, addr, len);
        let mut buf = [0; 4];

        self.cs.set_low();
        self.spi_write(&cmd.to_le_bytes());
        if func == FUNC_BACKPLANE {
            // 4-byte response delay.
            self.spi_read(&mut buf);
        }
        self.spi_read(&mut buf);
        self.cs.set_high();

        u32::from_le_bytes(buf)
    }

    fn writen(&mut self, func: u32, addr: u32, val: u32, len: u32) {
        let cmd = cmd_word(true, true, func, addr, len);

        self.cs.set_low();
        self.spi_write(&cmd.to_le_bytes());
        self.spi_write(&val.to_le_bytes());
        self.cs.set_high();
    }

    fn read32_swapped(&mut self, addr: u32) -> u32 {
        let cmd = cmd_word(false, true, FUNC_BUS, addr, 4);
        let mut buf = [0; 4];

        self.cs.set_low();
        self.spi_write(&swap16(cmd).to_le_bytes());
        self.spi_read(&mut buf);
        self.cs.set_high();

        swap16(u32::from_le_bytes(buf))
    }

    fn write32_swapped(&mut self, addr: u32, val: u32) {
        let cmd = cmd_word(true, true, FUNC_BUS, addr, 4);

        self.cs.set_low();
        self.spi_write(&swap16(cmd).to_le_bytes());
        self.spi_write(&swap16(val).to_le_bytes());
        self.cs.set_high();
    }

    fn spi_read(&mut self, words: &mut [u8]) {
        self.dio.set_as_input();
        for word in words {
            let mut w = 0;
            for _ in 0..8 {
                w = w << 1;

                // rising edge, sample data
                if self.dio.is_high() {
                    w |= 0x01;
                }
                self.clk.set_high();
                delay();

                // falling edge
                self.clk.set_low();
                delay();
            }
            *word = w
        }
        self.clk.set_low();
        delay();
    }

    fn spi_write(&mut self, words: &[u8]) {
        self.dio.set_as_output();
        for word in words {
            let mut word = *word;
            for _ in 0..8 {
                // falling edge, setup data
                self.clk.set_low();
                if word & 0x80 == 0 {
                    self.dio.set_low();
                } else {
                    self.dio.set_high();
                }
                delay();

                // rising edge
                self.clk.set_high();
                delay();

                word = word << 1;
            }
        }
        self.clk.set_low();
        delay();
        self.dio.set_as_input();
    }
}

fn delay() {
    //cortex_m::asm::delay(5);
}

macro_rules! nvram {
    ($($s:literal,)*) => {
        concat_bytes!($($s, b"\x00",)* b"\x00\x00")
    };
}

static NVRAM: &'static [u8] = &*nvram!(
    b"NVRAMRev=$Rev$",
    b"manfid=0x2d0",
    b"prodid=0x0727",
    b"vendid=0x14e4",
    b"devid=0x43e2",
    b"boardtype=0x0887",
    b"boardrev=0x1100",
    b"boardnum=22",
    b"macaddr=00:A0:50:86:aa:b6",
    b"sromrev=11",
    b"boardflags=0x00404001",
    b"boardflags3=0x04000000",
    b"xtalfreq=26000",
    b"nocrc=1",
    b"ag0=255",
    b"aa2g=1",
    b"ccode=ALL",
    b"pa0itssit=0x20",
    b"extpagain2g=0",
    b"pa2ga0=-168,7161,-820",
    b"AvVmid_c0=0x0,0xc8",
    b"cckpwroffset0=5",
    b"maxp2ga0=84",
    b"txpwrbckof=6",
    b"cckbw202gpo=0",
    b"legofdmbw202gpo=0x66111111",
    b"mcsbw202gpo=0x77711111",
    b"propbw202gpo=0xdd",
    b"ofdmdigfilttype=18",
    b"ofdmdigfilttypebe=18",
    b"papdmode=1",
    b"papdvalidtest=1",
    b"pacalidx2g=45",
    b"papdepsoffset=-30",
    b"papdendidx=58",
    b"ltecxmux=0",
    b"ltecxpadnum=0x0102",
    b"ltecxfnsel=0x44",
    b"ltecxgcigpio=0x01",
    b"il0macaddr=00:90:4c:c5:12:38",
    b"wl0id=0x431b",
    b"deadman_to=0xffffffff",
    b"muxenab=0x1",
    b"spurconfig=0x3",
    b"glitch_based_crsmin=1",
    b"btc_mode=1",
);
