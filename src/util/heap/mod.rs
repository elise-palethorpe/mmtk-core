#[macro_use]
pub mod layout;
// pub mod monotonepageresource;
pub mod pageresource2;
mod vmrequest;
// pub mod freelistpageresource;
pub mod space_descriptor;

// pub use self::monotonepageresource::MonotonePageResource;
pub use self::pageresource2::PageResource;
pub use self::vmrequest::VMRequest;
// pub use self::freelistpageresource::FreeListPageResource;