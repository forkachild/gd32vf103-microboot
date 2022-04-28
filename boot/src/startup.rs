extern "C" {
    #[link_name = "_sheap"]
    pub static start_heap: usize;
    #[link_name = "_eheap"]
    pub static end_heap: usize;

    pub fn abort() -> !;
}