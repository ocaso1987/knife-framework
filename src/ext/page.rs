use knife_util::VecExt;
use rbatis::Page;

pub trait PageExt<T> {
    fn transform<F, R>(&self, func: F) -> Page<R>
    where
        F: Fn(&T) -> R;
}

impl<T> PageExt<T> for Page<T> {
    fn transform<F, R>(&self, func: F) -> Page<R>
    where
        F: Fn(&T) -> R,
    {
        Page {
            records: self.records.map(func),
            total: self.total,
            pages: self.pages,
            page_no: self.page_no,
            page_size: self.page_size,
            search_count: self.search_count,
        }
    }
}
