#[doc = "Reader of register RELOAD0"]
pub type R = crate::R<u32, super::RELOAD0>;
#[doc = "Writer for register RELOAD0"]
pub type W = crate::W<u32, super::RELOAD0>;
#[doc = "Register RELOAD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RELOAD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reload`"]
pub type RELOAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reload`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
