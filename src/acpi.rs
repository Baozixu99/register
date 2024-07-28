extern crate bit_field;
use bit_field::BitField;
extern crate num_enum;
pub const ACPI_BASE: usize = 0x1FE2_7000;

const PM1_CNT_ADDR: usize = ACPI_BASE + 0x14;
macro_rules! impl_define_mem_reg {
    ($mem_reg_ident:ident,$mem_reg_addr:ident,$doc:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone)]
        pub struct $mem_reg_ident {
            bits: u32,
        }
        impl_write_mem_reg!($mem_reg_addr, $mem_reg_ident);
        impl_read_mem_reg!($mem_reg_addr, $mem_reg_ident);

        #[allow(unused)]
        impl $mem_reg_ident {
            pub fn get_mem_reg_offset() -> usize {
                $mem_reg_addr
            }
            pub fn empty() -> Self {
                Self { bits: 0 }
            }
            #[allow(unused)]
            pub fn set_value(&mut self, bit: u32) -> &mut Self {
                self.bits = bit;
                self
            }
            #[allow(unused)]
            pub fn get_value(&self) -> u32 {
                self.bits
            }
        }
        impl bit_field::BitField for $mem_reg_ident {
            const BIT_LENGTH: usize = usize::BIT_LENGTH;

            fn get_bit(&self, bit: usize) -> bool {
                self.bits.get_bit(bit)
            }

            fn get_bits<T: core::ops::RangeBounds<usize>>(&self, range: T) -> Self {
                Self {
                    bits: self.bits.get_bits(range),
                }
            }

            fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
                self.bits.set_bit(bit, value);
                self
            }

            fn set_bits<T: core::ops::RangeBounds<usize>>(
                &mut self,
                range: T,
                value: Self,
            ) -> &mut Self {
                self.bits.set_bits(range, value.bits);
                self
            }
        }
    };
}

macro_rules! impl_read_mem_reg {
    ($mem_reg_addr:ident,$mem_reg_ident:ident) => {
        impl $mem_reg_ident {
            #[inline(always)]
            #[allow(unused)]
            pub fn read(base: usize) -> $mem_reg_ident {
                $mem_reg_ident {
                    bits: unsafe { (($mem_reg_addr as *mut u32).read_volatile()) },
                }
            }
        }
    };
}

macro_rules! impl_write_mem_reg {
    ($mem_reg_addr:ident,$mem_reg_ident:ident) => {
        impl $mem_reg_ident {
            #[inline(always)]
            #[allow(unused)]
            pub fn write(&mut self) -> &mut Self {
                unsafe {
                    ($mem_reg_addr as *mut u32).write_volatile(self.bits);
                }
                self
            }
        }
    };
}

macro_rules! impl_get_set {
    ($mem_reg_get_ident:ident,$mem_reg_set_ident:ident,$num:literal,$doc:expr) => {
        #[doc = $doc]
        #[inline(always)]
        #[allow(unused)]
        pub fn $mem_reg_get_ident(&self) -> bool {
            self.get_bit($num)
        }

        #[doc = $doc]
        #[inline(always)]
        #[allow(unused)]
        pub fn $mem_reg_set_ident(&mut self, status: bool) -> &mut Self {
            self.set_bit($num, status);
            self
        }
    };
    
    ($mem_reg_get_ident:ident,$mem_reg_set_ident:ident,$range:expr,$doc:expr) => {
        #[doc = $doc]
        #[inline(always)]
        #[allow(unused)]
        pub fn $mem_reg_get_ident(&self) -> usize {
            self.get_bits($range).bits as usize
        }

        #[doc = $doc]
        #[inline(always)]
        #[allow(unused)]
        pub fn $mem_reg_set_ident(&mut self, status: usize) -> &mut Self {
            self.set_bits(
                $range,
                Self {
                    bits: status as u32,
                },
            );
            self
        }
    };
}
impl_define_mem_reg!(
    Pm1Cnt,
    PM1_CNT_ADDR,
    "电源管理1控制寄存器,<br>
    Power Management 1 Control Register "
);

impl Pm1Cnt {
    impl_get_set!(get_slp_en, set_slp_en, 13,
        "该位写1将会使系统进入SLP_TYP声明的休眠状态，进入相关休眠状态后该位自动恢复为0");
    impl_get_set!(get_slp_typ, set_slp_typ, 10..=12,
        "该3bit表示系统的休眠状态");
    /// 将系统设置为s5状态
    pub fn set_s5(&mut self) -> &mut Self{
        self.set_slp_typ(SleepType::S5.into());
        self.set_slp_en(true);
        self
    }
}

#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive, Debug)]
#[repr(usize)]
/// 该3bit表示系统的休眠状态
pub enum SleepType {
    /// 该模式下系统全部工作
    S0 = 0b000,
    /// Suspend to RAM(STR)，上下文保存到内存
    S3 = 0b101,
    /// Suspend to Disk(STD)，保存到硬盘，除唤醒电路全部掉电
    S4 = 0b110,
    /// Soft off，只有唤醒电路上电，“软关机”
    S5 = 0b111,
}