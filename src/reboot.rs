#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Provides support for rebooting the FPGA. You can select which of the four images to reboot to, just be sure to OR the image number with ``0xac``. For example, to reboot to the bootloader (image 0), write ``0xac``` to this register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Bits 24-31 of `REBOOT_ADDR`. This sets the reset vector for the VexRiscv. This address will be used whenever the CPU is reset, for example through a debug bridge. You should update this address whenever you load a new program, to enable the debugger to run ``mon reset``"]
    pub addr3: ADDR3,
    #[doc = "0x08 - Bits 16-23 of `REBOOT_ADDR`."]
    pub addr2: ADDR2,
    #[doc = "0x0c - Bits 8-15 of `REBOOT_ADDR`."]
    pub addr1: ADDR1,
    #[doc = "0x10 - Bits 0-7 of `REBOOT_ADDR`."]
    pub addr0: ADDR0,
}
#[doc = "Provides support for rebooting the FPGA. You can select which of the four images to reboot to, just be sure to OR the image number with ``0xac``. For example, to reboot to the bootloader (image 0), write ``0xac``` to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Provides support for rebooting the FPGA. You can select which of the four images to reboot to, just be sure to OR the image number with ``0xac``. For example, to reboot to the bootloader (image 0), write ``0xac``` to this register."]
pub mod ctrl;
#[doc = "Bits 24-31 of `REBOOT_ADDR`. This sets the reset vector for the VexRiscv. This address will be used whenever the CPU is reset, for example through a debug bridge. You should update this address whenever you load a new program, to enable the debugger to run ``mon reset``\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr3](addr3) module"]
pub type ADDR3 = crate::Reg<u32, _ADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR3;
#[doc = "`read()` method returns [addr3::R](addr3::R) reader structure"]
impl crate::Readable for ADDR3 {}
#[doc = "`write(|w| ..)` method takes [addr3::W](addr3::W) writer structure"]
impl crate::Writable for ADDR3 {}
#[doc = "Bits 24-31 of `REBOOT_ADDR`. This sets the reset vector for the VexRiscv. This address will be used whenever the CPU is reset, for example through a debug bridge. You should update this address whenever you load a new program, to enable the debugger to run ``mon reset``"]
pub mod addr3;
#[doc = "Bits 16-23 of `REBOOT_ADDR`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr2](addr2) module"]
pub type ADDR2 = crate::Reg<u32, _ADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR2;
#[doc = "`read()` method returns [addr2::R](addr2::R) reader structure"]
impl crate::Readable for ADDR2 {}
#[doc = "`write(|w| ..)` method takes [addr2::W](addr2::W) writer structure"]
impl crate::Writable for ADDR2 {}
#[doc = "Bits 16-23 of `REBOOT_ADDR`."]
pub mod addr2;
#[doc = "Bits 8-15 of `REBOOT_ADDR`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr1](addr1) module"]
pub type ADDR1 = crate::Reg<u32, _ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR1;
#[doc = "`read()` method returns [addr1::R](addr1::R) reader structure"]
impl crate::Readable for ADDR1 {}
#[doc = "`write(|w| ..)` method takes [addr1::W](addr1::W) writer structure"]
impl crate::Writable for ADDR1 {}
#[doc = "Bits 8-15 of `REBOOT_ADDR`."]
pub mod addr1;
#[doc = "Bits 0-7 of `REBOOT_ADDR`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr0](addr0) module"]
pub type ADDR0 = crate::Reg<u32, _ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0;
#[doc = "`read()` method returns [addr0::R](addr0::R) reader structure"]
impl crate::Readable for ADDR0 {}
#[doc = "`write(|w| ..)` method takes [addr0::W](addr0::W) writer structure"]
impl crate::Writable for ADDR0 {}
#[doc = "Bits 0-7 of `REBOOT_ADDR`."]
pub mod addr0;
