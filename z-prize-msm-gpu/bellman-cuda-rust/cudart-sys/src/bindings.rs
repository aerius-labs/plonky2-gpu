/* automatically generated by rust-bindgen 0.59.2 */

pub const cudaHostAllocDefault: u32 = 0;
pub const cudaHostAllocPortable: u32 = 1;
pub const cudaHostAllocMapped: u32 = 2;
pub const cudaHostAllocWriteCombined: u32 = 4;
pub const cudaHostRegisterDefault: u32 = 0;
pub const cudaHostRegisterPortable: u32 = 1;
pub const cudaHostRegisterMapped: u32 = 2;
pub const cudaHostRegisterIoMemory: u32 = 4;
pub const cudaHostRegisterReadOnly: u32 = 8;
pub const cudaStreamDefault: u32 = 0;
pub const cudaStreamNonBlocking: u32 = 1;
pub const cudaEventDefault: u32 = 0;
pub const cudaEventBlockingSync: u32 = 1;
pub const cudaEventDisableTiming: u32 = 2;
pub const cudaEventInterprocess: u32 = 4;
pub const cudaEventRecordDefault: u32 = 0;
pub const cudaEventRecordExternal: u32 = 1;
pub const cudaEventWaitDefault: u32 = 0;
pub const cudaEventWaitExternal: u32 = 1;
#[repr(u32)]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaError {
    Success = 0,
    ErrorInvalidValue = 1,
    ErrorMemoryAllocation = 2,
    ErrorInitializationError = 3,
    ErrorCudartUnloading = 4,
    ErrorProfilerDisabled = 5,
    ErrorProfilerNotInitialized = 6,
    ErrorProfilerAlreadyStarted = 7,
    ErrorProfilerAlreadyStopped = 8,
    ErrorInvalidConfiguration = 9,
    ErrorInvalidPitchValue = 12,
    ErrorInvalidSymbol = 13,
    ErrorInvalidHostPointer = 16,
    ErrorInvalidDevicePointer = 17,
    ErrorInvalidTexture = 18,
    ErrorInvalidTextureBinding = 19,
    ErrorInvalidChannelDescriptor = 20,
    ErrorInvalidMemcpyDirection = 21,
    ErrorAddressOfConstant = 22,
    ErrorTextureFetchFailed = 23,
    ErrorTextureNotBound = 24,
    ErrorSynchronizationError = 25,
    ErrorInvalidFilterSetting = 26,
    ErrorInvalidNormSetting = 27,
    ErrorMixedDeviceExecution = 28,
    ErrorNotYetImplemented = 31,
    ErrorMemoryValueTooLarge = 32,
    ErrorStubLibrary = 34,
    ErrorInsufficientDriver = 35,
    ErrorCallRequiresNewerDriver = 36,
    ErrorInvalidSurface = 37,
    ErrorDuplicateVariableName = 43,
    ErrorDuplicateTextureName = 44,
    ErrorDuplicateSurfaceName = 45,
    ErrorDevicesUnavailable = 46,
    ErrorIncompatibleDriverContext = 49,
    ErrorMissingConfiguration = 52,
    ErrorPriorLaunchFailure = 53,
    ErrorLaunchMaxDepthExceeded = 65,
    ErrorLaunchFileScopedTex = 66,
    ErrorLaunchFileScopedSurf = 67,
    ErrorSyncDepthExceeded = 68,
    ErrorLaunchPendingCountExceeded = 69,
    ErrorInvalidDeviceFunction = 98,
    ErrorNoDevice = 100,
    ErrorInvalidDevice = 101,
    ErrorDeviceNotLicensed = 102,
    ErrorSoftwareValidityNotEstablished = 103,
    ErrorStartupFailure = 127,
    ErrorInvalidKernelImage = 200,
    ErrorDeviceUninitialized = 201,
    ErrorMapBufferObjectFailed = 205,
    ErrorUnmapBufferObjectFailed = 206,
    ErrorArrayIsMapped = 207,
    ErrorAlreadyMapped = 208,
    ErrorNoKernelImageForDevice = 209,
    ErrorAlreadyAcquired = 210,
    ErrorNotMapped = 211,
    ErrorNotMappedAsArray = 212,
    ErrorNotMappedAsPointer = 213,
    ErrorECCUncorrectable = 214,
    ErrorUnsupportedLimit = 215,
    ErrorDeviceAlreadyInUse = 216,
    ErrorPeerAccessUnsupported = 217,
    ErrorInvalidPtx = 218,
    ErrorInvalidGraphicsContext = 219,
    ErrorNvlinkUncorrectable = 220,
    ErrorJitCompilerNotFound = 221,
    ErrorUnsupportedPtxVersion = 222,
    ErrorJitCompilationDisabled = 223,
    ErrorUnsupportedExecAffinity = 224,
    ErrorInvalidSource = 300,
    ErrorFileNotFound = 301,
    ErrorSharedObjectSymbolNotFound = 302,
    ErrorSharedObjectInitFailed = 303,
    ErrorOperatingSystem = 304,
    ErrorInvalidResourceHandle = 400,
    ErrorIllegalState = 401,
    ErrorSymbolNotFound = 500,
    ErrorNotReady = 600,
    ErrorIllegalAddress = 700,
    ErrorLaunchOutOfResources = 701,
    ErrorLaunchTimeout = 702,
    ErrorLaunchIncompatibleTexturing = 703,
    ErrorPeerAccessAlreadyEnabled = 704,
    ErrorPeerAccessNotEnabled = 705,
    ErrorSetOnActiveProcess = 708,
    ErrorContextIsDestroyed = 709,
    ErrorAssert = 710,
    ErrorTooManyPeers = 711,
    ErrorHostMemoryAlreadyRegistered = 712,
    ErrorHostMemoryNotRegistered = 713,
    ErrorHardwareStackError = 714,
    ErrorIllegalInstruction = 715,
    ErrorMisalignedAddress = 716,
    ErrorInvalidAddressSpace = 717,
    ErrorInvalidPc = 718,
    ErrorLaunchFailure = 719,
    ErrorCooperativeLaunchTooLarge = 720,
    ErrorNotPermitted = 800,
    ErrorNotSupported = 801,
    ErrorSystemNotReady = 802,
    ErrorSystemDriverMismatch = 803,
    ErrorCompatNotSupportedOnDevice = 804,
    ErrorMpsConnectionFailed = 805,
    ErrorMpsRpcFailure = 806,
    ErrorMpsServerNotReady = 807,
    ErrorMpsMaxClientsReached = 808,
    ErrorMpsMaxConnectionsReached = 809,
    ErrorMpsClientTerminated = 810,
    ErrorCdpNotSupported = 811,
    ErrorCdpVersionMismatch = 812,
    ErrorStreamCaptureUnsupported = 900,
    ErrorStreamCaptureInvalidated = 901,
    ErrorStreamCaptureMerge = 902,
    ErrorStreamCaptureUnmatched = 903,
    ErrorStreamCaptureUnjoined = 904,
    ErrorStreamCaptureIsolation = 905,
    ErrorStreamCaptureImplicit = 906,
    ErrorCapturedEvent = 907,
    ErrorStreamCaptureWrongThread = 908,
    ErrorTimeout = 909,
    ErrorGraphExecUpdateFailure = 910,
    ErrorExternalDevice = 911,
    ErrorInvalidClusterSize = 912,
    ErrorUnknown = 999,
    ErrorApiFailureBase = 10000,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemoryType {
    Unregistered = 0,
    Host = 1,
    Device = 2,
    Managed = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemoryCopyKind {
    HostToHost = 0,
    HostToDevice = 1,
    DeviceToHost = 2,
    DeviceToDevice = 3,
    Default = 4,
}
pub type cudaHostFn_t =
    ::std::option::Option<unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CudaPointerAttributes {
    pub type_: CudaMemoryType,
    pub device: ::std::os::raw::c_int,
    pub devicePointer: *mut ::std::os::raw::c_void,
    pub hostPointer: *mut ::std::os::raw::c_void,
}
impl CudaDeviceAttr {
    pub const MaxTimelineSemaphoreInteropSupported: CudaDeviceAttr =
        CudaDeviceAttr::TimelineSemaphoreInteropSupported;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaDeviceAttr {
    MaxThreadsPerBlock = 1,
    MaxBlockDimX = 2,
    MaxBlockDimY = 3,
    MaxBlockDimZ = 4,
    MaxGridDimX = 5,
    MaxGridDimY = 6,
    MaxGridDimZ = 7,
    MaxSharedMemoryPerBlock = 8,
    TotalConstantMemory = 9,
    WarpSize = 10,
    MaxPitch = 11,
    MaxRegistersPerBlock = 12,
    ClockRate = 13,
    TextureAlignment = 14,
    GpuOverlap = 15,
    MultiProcessorCount = 16,
    KernelExecTimeout = 17,
    Integrated = 18,
    CanMapHostMemory = 19,
    ComputeMode = 20,
    MaxTexture1DWidth = 21,
    MaxTexture2DWidth = 22,
    MaxTexture2DHeight = 23,
    MaxTexture3DWidth = 24,
    MaxTexture3DHeight = 25,
    MaxTexture3DDepth = 26,
    MaxTexture2DLayeredWidth = 27,
    MaxTexture2DLayeredHeight = 28,
    MaxTexture2DLayeredLayers = 29,
    SurfaceAlignment = 30,
    ConcurrentKernels = 31,
    EccEnabled = 32,
    PciBusId = 33,
    PciDeviceId = 34,
    TccDriver = 35,
    MemoryClockRate = 36,
    GlobalMemoryBusWidth = 37,
    L2CacheSize = 38,
    MaxThreadsPerMultiProcessor = 39,
    AsyncEngineCount = 40,
    UnifiedAddressing = 41,
    MaxTexture1DLayeredWidth = 42,
    MaxTexture1DLayeredLayers = 43,
    MaxTexture2DGatherWidth = 45,
    MaxTexture2DGatherHeight = 46,
    MaxTexture3DWidthAlt = 47,
    MaxTexture3DHeightAlt = 48,
    MaxTexture3DDepthAlt = 49,
    PciDomainId = 50,
    TexturePitchAlignment = 51,
    MaxTextureCubemapWidth = 52,
    MaxTextureCubemapLayeredWidth = 53,
    MaxTextureCubemapLayeredLayers = 54,
    MaxSurface1DWidth = 55,
    MaxSurface2DWidth = 56,
    MaxSurface2DHeight = 57,
    MaxSurface3DWidth = 58,
    MaxSurface3DHeight = 59,
    MaxSurface3DDepth = 60,
    MaxSurface1DLayeredWidth = 61,
    MaxSurface1DLayeredLayers = 62,
    MaxSurface2DLayeredWidth = 63,
    MaxSurface2DLayeredHeight = 64,
    MaxSurface2DLayeredLayers = 65,
    MaxSurfaceCubemapWidth = 66,
    MaxSurfaceCubemapLayeredWidth = 67,
    MaxSurfaceCubemapLayeredLayers = 68,
    MaxTexture1DLinearWidth = 69,
    MaxTexture2DLinearWidth = 70,
    MaxTexture2DLinearHeight = 71,
    MaxTexture2DLinearPitch = 72,
    MaxTexture2DMipmappedWidth = 73,
    MaxTexture2DMipmappedHeight = 74,
    ComputeCapabilityMajor = 75,
    ComputeCapabilityMinor = 76,
    MaxTexture1DMipmappedWidth = 77,
    StreamPrioritiesSupported = 78,
    GlobalL1CacheSupported = 79,
    LocalL1CacheSupported = 80,
    MaxSharedMemoryPerMultiprocessor = 81,
    MaxRegistersPerMultiprocessor = 82,
    ManagedMemory = 83,
    IsMultiGpuBoard = 84,
    MultiGpuBoardGroupID = 85,
    HostNativeAtomicSupported = 86,
    SingleToDoublePrecisionPerfRatio = 87,
    PageableMemoryAccess = 88,
    ConcurrentManagedAccess = 89,
    ComputePreemptionSupported = 90,
    CanUseHostPointerForRegisteredMem = 91,
    Reserved92 = 92,
    Reserved93 = 93,
    Reserved94 = 94,
    CooperativeLaunch = 95,
    CooperativeMultiDeviceLaunch = 96,
    MaxSharedMemoryPerBlockOptin = 97,
    CanFlushRemoteWrites = 98,
    HostRegisterSupported = 99,
    PageableMemoryAccessUsesHostPageTables = 100,
    DirectManagedMemAccessFromHost = 101,
    MaxBlocksPerMultiprocessor = 106,
    MaxPersistingL2CacheSize = 108,
    MaxAccessPolicyWindowSize = 109,
    ReservedSharedMemoryPerBlock = 111,
    SparseCudaArraySupported = 112,
    HostRegisterReadOnlySupported = 113,
    TimelineSemaphoreInteropSupported = 114,
    MemoryPoolsSupported = 115,
    GPUDirectRDMASupported = 116,
    GPUDirectRDMAFlushWritesOptions = 117,
    GPUDirectRDMAWritesOrdering = 118,
    MemoryPoolSupportedHandleTypes = 119,
    ClusterLaunch = 120,
    DeferredMappingCudaArraySupported = 121,
    Reserved122 = 122,
    Reserved123 = 123,
    Reserved124 = 124,
    IpcEventSupport = 125,
    MemSyncDomainCount = 126,
    Max = 127,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemPoolAttribute {
    ReuseFollowEventDependencies = 1,
    ReuseAllowOpportunistic = 2,
    ReuseAllowInternalDependencies = 3,
    AttrReleaseThreshold = 4,
    AttrReservedMemCurrent = 5,
    AttrReservedMemHigh = 6,
    AttrUsedMemCurrent = 7,
    AttrUsedMemHigh = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemLocationType {
    Invalid = 0,
    Device = 1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CudaMemLocation {
    pub type_: CudaMemLocationType,
    pub id: ::std::os::raw::c_int,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemAccessFlags {
    None = 0,
    Read = 1,
    ReadWrite = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CudaMemAccessDesc {
    pub location: CudaMemLocation,
    pub flags: CudaMemAccessFlags,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemAllocationType {
    Invalid = 0,
    Pinned = 1,
    Max = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CudaMemAllocationHandleType {
    None = 0,
    PosixFileDescriptor = 1,
    Win32 = 2,
    Win32Kmt = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CudaMemPoolProperties {
    pub allocType: CudaMemAllocationType,
    pub handleTypes: CudaMemAllocationHandleType,
    pub location: CudaMemLocation,
    pub win32SecurityAttributes: *mut ::std::os::raw::c_void,
    pub reserved: [::std::os::raw::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUuuid_st {
    pub bytes: [::std::os::raw::c_char; 16usize],
}
pub type cudaUUID_t = CUuuid_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CudaDeviceProperties {
    pub name: [::std::os::raw::c_char; 256usize],
    pub uuid: cudaUUID_t,
    pub luid: [::std::os::raw::c_char; 8usize],
    pub luidDeviceNodeMask: ::std::os::raw::c_uint,
    pub totalGlobalMem: usize,
    pub sharedMemPerBlock: usize,
    pub regsPerBlock: ::std::os::raw::c_int,
    pub warpSize: ::std::os::raw::c_int,
    pub memPitch: usize,
    pub maxThreadsPerBlock: ::std::os::raw::c_int,
    pub maxThreadsDim: [::std::os::raw::c_int; 3usize],
    pub maxGridSize: [::std::os::raw::c_int; 3usize],
    pub clockRate: ::std::os::raw::c_int,
    pub totalConstMem: usize,
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub textureAlignment: usize,
    pub texturePitchAlignment: usize,
    pub deviceOverlap: ::std::os::raw::c_int,
    pub multiProcessorCount: ::std::os::raw::c_int,
    pub kernelExecTimeoutEnabled: ::std::os::raw::c_int,
    pub integrated: ::std::os::raw::c_int,
    pub canMapHostMemory: ::std::os::raw::c_int,
    pub computeMode: ::std::os::raw::c_int,
    pub maxTexture1D: ::std::os::raw::c_int,
    pub maxTexture1DMipmap: ::std::os::raw::c_int,
    pub maxTexture1DLinear: ::std::os::raw::c_int,
    pub maxTexture2D: [::std::os::raw::c_int; 2usize],
    pub maxTexture2DMipmap: [::std::os::raw::c_int; 2usize],
    pub maxTexture2DLinear: [::std::os::raw::c_int; 3usize],
    pub maxTexture2DGather: [::std::os::raw::c_int; 2usize],
    pub maxTexture3D: [::std::os::raw::c_int; 3usize],
    pub maxTexture3DAlt: [::std::os::raw::c_int; 3usize],
    pub maxTextureCubemap: ::std::os::raw::c_int,
    pub maxTexture1DLayered: [::std::os::raw::c_int; 2usize],
    pub maxTexture2DLayered: [::std::os::raw::c_int; 3usize],
    pub maxTextureCubemapLayered: [::std::os::raw::c_int; 2usize],
    pub maxSurface1D: ::std::os::raw::c_int,
    pub maxSurface2D: [::std::os::raw::c_int; 2usize],
    pub maxSurface3D: [::std::os::raw::c_int; 3usize],
    pub maxSurface1DLayered: [::std::os::raw::c_int; 2usize],
    pub maxSurface2DLayered: [::std::os::raw::c_int; 3usize],
    pub maxSurfaceCubemap: ::std::os::raw::c_int,
    pub maxSurfaceCubemapLayered: [::std::os::raw::c_int; 2usize],
    pub surfaceAlignment: usize,
    pub concurrentKernels: ::std::os::raw::c_int,
    pub ECCEnabled: ::std::os::raw::c_int,
    pub pciBusID: ::std::os::raw::c_int,
    pub pciDeviceID: ::std::os::raw::c_int,
    pub pciDomainID: ::std::os::raw::c_int,
    pub tccDriver: ::std::os::raw::c_int,
    pub asyncEngineCount: ::std::os::raw::c_int,
    pub unifiedAddressing: ::std::os::raw::c_int,
    pub memoryClockRate: ::std::os::raw::c_int,
    pub memoryBusWidth: ::std::os::raw::c_int,
    pub l2CacheSize: ::std::os::raw::c_int,
    pub persistingL2CacheMaxSize: ::std::os::raw::c_int,
    pub maxThreadsPerMultiProcessor: ::std::os::raw::c_int,
    pub streamPrioritiesSupported: ::std::os::raw::c_int,
    pub globalL1CacheSupported: ::std::os::raw::c_int,
    pub localL1CacheSupported: ::std::os::raw::c_int,
    pub sharedMemPerMultiprocessor: usize,
    pub regsPerMultiprocessor: ::std::os::raw::c_int,
    pub managedMemory: ::std::os::raw::c_int,
    pub isMultiGpuBoard: ::std::os::raw::c_int,
    pub multiGpuBoardGroupID: ::std::os::raw::c_int,
    pub hostNativeAtomicSupported: ::std::os::raw::c_int,
    pub singleToDoublePrecisionPerfRatio: ::std::os::raw::c_int,
    pub pageableMemoryAccess: ::std::os::raw::c_int,
    pub concurrentManagedAccess: ::std::os::raw::c_int,
    pub computePreemptionSupported: ::std::os::raw::c_int,
    pub canUseHostPointerForRegisteredMem: ::std::os::raw::c_int,
    pub cooperativeLaunch: ::std::os::raw::c_int,
    pub cooperativeMultiDeviceLaunch: ::std::os::raw::c_int,
    pub sharedMemPerBlockOptin: usize,
    pub pageableMemoryAccessUsesHostPageTables: ::std::os::raw::c_int,
    pub directManagedMemAccessFromHost: ::std::os::raw::c_int,
    pub maxBlocksPerMultiProcessor: ::std::os::raw::c_int,
    pub accessPolicyMaxWindowSize: ::std::os::raw::c_int,
    pub reservedSharedMemPerBlock: usize,
    pub hostRegisterSupported: ::std::os::raw::c_int,
    pub sparseCudaArraySupported: ::std::os::raw::c_int,
    pub hostRegisterReadOnlySupported: ::std::os::raw::c_int,
    pub timelineSemaphoreInteropSupported: ::std::os::raw::c_int,
    pub memoryPoolsSupported: ::std::os::raw::c_int,
    pub gpuDirectRDMASupported: ::std::os::raw::c_int,
    pub gpuDirectRDMAFlushWritesOptions: ::std::os::raw::c_uint,
    pub gpuDirectRDMAWritesOrdering: ::std::os::raw::c_int,
    pub memoryPoolSupportedHandleTypes: ::std::os::raw::c_uint,
    pub deferredMappingCudaArraySupported: ::std::os::raw::c_int,
    pub ipcEventSupported: ::std::os::raw::c_int,
    pub clusterLaunch: ::std::os::raw::c_int,
    pub unifiedFunctionPointers: ::std::os::raw::c_int,
    pub reserved: [::std::os::raw::c_int; 63usize],
}
pub use self::CudaError as cudaError_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type cudaStream_t = *mut CUstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUevent_st {
    _unused: [u8; 0],
}
pub type cudaEvent_t = *mut CUevent_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemPoolHandle_st {
    _unused: [u8; 0],
}
pub type cudaMemPool_t = *mut CUmemPoolHandle_st;
extern "C" {
    pub fn cudaDeviceReset() -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSynchronize() -> cudaError_t;
}
extern "C" {
    pub fn cudaGetLastError() -> cudaError_t;
}
extern "C" {
    pub fn cudaPeekAtLastError() -> cudaError_t;
}
extern "C" {
    pub fn cudaGetErrorName(error: cudaError_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn cudaGetDeviceCount(count: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    #[link_name = "\u{1}cudaGetDeviceProperties_v2"]
    pub fn cudaGetDeviceProperties(
        prop: *mut CudaDeviceProperties,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetAttribute(
        value: *mut ::std::os::raw::c_int,
        attr: CudaDeviceAttr,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetDefaultMemPool(
        memPool: *mut cudaMemPool_t,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceSetMemPool(
        device: ::std::os::raw::c_int,
        memPool: cudaMemPool_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceGetMemPool(
        memPool: *mut cudaMemPool_t,
        device: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaSetDevice(device: ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaGetDevice(device: *mut ::std::os::raw::c_int) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreate(pStream: *mut cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamCreateWithFlags(
        pStream: *mut cudaStream_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamDestroy(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamWaitEvent(
        stream: cudaStream_t,
        event: cudaEvent_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamSynchronize(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaStreamQuery(stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventCreate(event: *mut cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventCreateWithFlags(
        event: *mut cudaEvent_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventRecord(event: cudaEvent_t, stream: cudaStream_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventRecordWithFlags(
        event: cudaEvent_t,
        stream: cudaStream_t,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventQuery(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventSynchronize(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventDestroy(event: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaEventElapsedTime(ms: *mut f32, start: cudaEvent_t, end: cudaEvent_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaLaunchHostFunc(
        stream: cudaStream_t,
        fn_: cudaHostFn_t,
        userData: *mut ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMalloc(devPtr: *mut *mut ::std::os::raw::c_void, size: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaFree(devPtr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaFreeHost(ptr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostAlloc(
        pHost: *mut *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostRegister(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaHostUnregister(ptr: *mut ::std::os::raw::c_void) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemGetInfo(free: *mut usize, total: *mut usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpy(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: CudaMemoryCopyKind,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemcpyAsync(
        dst: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
        count: usize,
        kind: CudaMemoryCopyKind,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemset(
        devPtr: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemsetAsync(
        devPtr: *mut ::std::os::raw::c_void,
        value: ::std::os::raw::c_int,
        count: usize,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocAsync(
        devPtr: *mut *mut ::std::os::raw::c_void,
        size: usize,
        hStream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaFreeAsync(devPtr: *mut ::std::os::raw::c_void, hStream: cudaStream_t)
        -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolTrimTo(memPool: cudaMemPool_t, minBytesToKeep: usize) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolSetAttribute(
        memPool: cudaMemPool_t,
        attr: CudaMemPoolAttribute,
        value: *mut ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolGetAttribute(
        memPool: cudaMemPool_t,
        attr: CudaMemPoolAttribute,
        value: *mut ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolSetAccess(
        memPool: cudaMemPool_t,
        descList: *const CudaMemAccessDesc,
        count: usize,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolGetAccess(
        flags: *mut CudaMemAccessFlags,
        memPool: cudaMemPool_t,
        location: *mut CudaMemLocation,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolCreate(
        memPool: *mut cudaMemPool_t,
        poolProps: *const CudaMemPoolProperties,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaMemPoolDestroy(memPool: cudaMemPool_t) -> cudaError_t;
}
extern "C" {
    pub fn cudaMallocFromPoolAsync(
        ptr: *mut *mut ::std::os::raw::c_void,
        size: usize,
        memPool: cudaMemPool_t,
        stream: cudaStream_t,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaPointerGetAttributes(
        attributes: *mut CudaPointerAttributes,
        ptr: *const ::std::os::raw::c_void,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceCanAccessPeer(
        canAccessPeer: *mut ::std::os::raw::c_int,
        device: ::std::os::raw::c_int,
        peerDevice: ::std::os::raw::c_int,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceEnablePeerAccess(
        peerDevice: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_uint,
    ) -> cudaError_t;
}
extern "C" {
    pub fn cudaDeviceDisablePeerAccess(peerDevice: ::std::os::raw::c_int) -> cudaError_t;
}
