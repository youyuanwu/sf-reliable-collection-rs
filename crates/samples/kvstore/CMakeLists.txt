#proto
message(STATUS "fetching protoc")
include(FetchContent)
if(WIN32)
  FetchContent_Declare(
    protoc
    DOWNLOAD_NO_EXTRACT FALSE
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
    URL https://github.com/protocolbuffers/protobuf/releases/download/v24.1/protoc-24.1-win64.zip
    URL_HASH SHA256=DAD1EFED75CB772A0C9E5D4646006BF87E1E051191473A2A30C9DB66EE223CFD
  )
else(WIN32)
  FetchContent_Declare(
    protoc
    DOWNLOAD_NO_EXTRACT FALSE
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
    URL https://github.com/protocolbuffers/protobuf/releases/download/v26.0/protoc-26.0-linux-x86_64.zip
  )
endif(WIN32)
FetchContent_GetProperties(protoc)
if(NOT protoc_POPULATED)
  FetchContent_Populate(protoc)
endif()


# copy files to build folder to form a code package.

add_custom_target(build_rust_sample_kvstore ALL
    COMMAND ${cargo_exe} build -p kvstore
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

set(_pkg_root ${CMAKE_BINARY_DIR}/kvstore_root)
set(_pkg_src  ${CMAKE_SOURCE_DIR}/crates/samples/kvstore)

set(_pkg_exe  ${CMAKE_SOURCE_DIR}/target/debug/kvstore.exe)
add_custom_command(TARGET build_rust_sample_kvstore POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory ${_pkg_root}
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_src}/manifests/KvStoreServicePackage/ServiceManifest.xml ${_pkg_root}/KvStoreServicePackage/ServiceManifest.xml
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_src}/manifests/ApplicationManifest.xml ${_pkg_root}/ApplicationManifest.xml
    COMMAND ${CMAKE_COMMAND} 
    -E copy_if_different ${_pkg_exe} ${_pkg_root}/KvStoreServicePackage/Code/kvstore.exe
)