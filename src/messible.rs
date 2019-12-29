#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write half of the FIFO to send data out the Messible. Writing to this register advances the write pointer automatically."]
    pub in_: IN,
    #[doc = "0x04 - Read half of the FIFO to receive data on the Messible. Reading from this register advances the read pointer automatically."]
    pub out: OUT,
    #[doc = "0x08 - "]
    pub status: STATUS,
}
#[doc = "Write half of the FIFO to send data out the Messible. Writing to this register advances the write pointer automatically.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "`write(|w| ..)` method takes [in_::W](in_::W) writer structure"]
impl crate::Writable for IN {}
#[doc = "Write half of the FIFO to send data out the Messible. Writing to this register advances the write pointer automatically."]
pub mod in_;
#[doc = "Read half of the FIFO to receive data on the Messible. Reading from this register advances the read pointer automatically.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "Read half of the FIFO to receive data on the Messible. Reading from this register advances the read pointer automatically."]
pub mod out;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = ""]
pub mod status;
