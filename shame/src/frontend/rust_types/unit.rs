use crate::frontend::any::{Any, InvalidReason};
use crate::frontend::encoding::buffer::BufferAddressSpace;
use crate::frontend::encoding::buffer::{BufferInner, BufferRefInner};
use crate::frontend::any::shared_io::BindingType;
use crate::frontend::rust_types::layout_traits::{ArrayElementsUnsizedError, FromAnys, GetAllFields, GpuLayout};
use crate::frontend::rust_types::mem::AddressSpace;
use crate::frontend::rust_types::reference::{AccessMode, AccessModeReadable};
use crate::frontend::rust_types::struct_::{BufferFields, SizedFields};
use crate::frontend::rust_types::type_layout::{TypeLayout, TypeLayoutRules, TypeLayoutSemantics};
use crate::frontend::rust_types::type_traits::{
    BindingArgs, EmptyRefFields, GpuAligned, GpuSized, GpuStore, GpuStoreImplCategory,
    NoAtomics, NoBools, NoHandles,
};
use crate::frontend::rust_types::{AsAny, GpuType, ToGpuType};
use crate::ir::{self, AlignedType, Len, ScalarType, SizedType, StoreType, Type};
use std::borrow::{Borrow, Cow};

// ---------------------------------------------------------------------------
// () – the unit type as a ZST GpuType
// ---------------------------------------------------------------------------

impl GpuLayout for () {
    const IS_ZST: bool = true;

    fn gpu_layout() -> TypeLayout {
        // ZST has no meaningful GPU layout; we report size=0, align=1
        TypeLayout::new(Some(0), 1, TypeLayoutSemantics::Vector(Len::X1, ScalarType::U32))
    }

    fn cpu_type_name_and_layout(
    ) -> Option<Result<(Cow<'static, str>, TypeLayout), ArrayElementsUnsizedError>> {
        None
    }
}

impl GpuSized for () {
    fn sized_ty() -> Option<SizedType>
    where
        Self: GpuType,
    {
        None
    }
}

impl GpuAligned for () {
    fn aligned_ty() -> Option<AlignedType>
    where
        Self: GpuType,
    {
        None
    }
}

impl NoBools for () {}
impl NoAtomics for () {}
impl NoHandles for () {}

impl FromAnys for () {
    fn expected_num_anys() -> usize {
        0
    }

    fn from_anys(anys: impl Iterator<Item = Any>) -> Self {
        let _ = anys;
    }
}

impl GetAllFields for () {
    fn fields_as_anys_unchecked(self_as_any: Any) -> impl Borrow<[Any]> {
        let _ = self_as_any;
        [] as [Any; 0]
    }
}

impl GpuStore for () {
    type RefFields<AS: AddressSpace, AM: AccessMode> = EmptyRefFields;

    fn store_ty() -> Option<StoreType>
    where
        Self: GpuType,
    {
        None
    }

    fn instantiate_buffer_inner<AS: BufferAddressSpace>(
        _args: Result<BindingArgs, InvalidReason>,
        _bind_ty: BindingType,
    ) -> BufferInner<Self, AS>
    where
        Self: NoAtomics + NoBools,
    {
        unreachable!("ZST buffer guarded by IS_ZST check")
    }

    fn instantiate_buffer_ref_inner<AS: BufferAddressSpace, AM: AccessModeReadable>(
        _args: Result<BindingArgs, InvalidReason>,
        _bind_ty: BindingType,
    ) -> BufferRefInner<Self, AS, AM>
    where
        Self: NoBools,
    {
        unreachable!("ZST buffer ref guarded by IS_ZST check")
    }

    fn impl_category() -> GpuStoreImplCategory {
        GpuStoreImplCategory::GpuType(None)
    }
}

impl GpuType for () {
    fn ty() -> Type {
        // ZST has no IR type; callers should check IS_ZST before calling ty()
        Type::Unit
    }

    fn from_any_unchecked(any: Any) -> Self {
        let _ = any;
    }
}

impl AsAny for () {
    fn as_any(&self) -> Any {
        Any::new_invalid(InvalidReason::CreatedWithNoActiveEncoding)
    }
}

impl ToGpuType for () {
    type Gpu = ();

    fn to_gpu(&self) -> Self::Gpu {}

    fn as_gpu_type_ref(&self) -> Option<&Self::Gpu> {
        Some(self)
    }
}

impl From<Any> for () {
    fn from(any: Any) -> Self {
        let _ = any;
    }
}
