mod iallocator;
mod allocator;
mod wasm_allocator;
mod mapper;

pub use iallocator::IAllocator;
pub use allocator::Allocator;
pub use wasm_allocator::WasmAllocator;
pub use mapper::SizeToWasmBlockMapper;