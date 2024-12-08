
#[repr(C, align(4096))]
pub struct Page(pub [u8; 4096]);

impl Page {
    pub(crate) const ZERO: Page = Page([0; 4096]);
}
