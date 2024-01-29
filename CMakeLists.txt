cmake_minimum_required(VERSION 3.20)

project(sf-reliable-collection-rs LANGUAGES)

message(STATUS "fetching fabric_metadata")
include(FetchContent)
FetchContent_Declare(fabric_metadata
    GIT_REPOSITORY https://github.com/youyuanwu/fabric-metadata.git
    GIT_TAG b070db62d2323d85d402ca3f6dc387f0ce0be37b
)
FetchContent_GetProperties(fabric_metadata)
if(NOT fabric_metadata_POPULATED)
    FetchContent_Populate(fabric_metadata)
    # do not add to cmake build since we only need winmd file to generate code.
endif()

include(${fabric_metadata_SOURCE_DIR}/cmake/ImportFabric.cmake)
message(STATUS "Fabric Code Root: ${ServiceFabric_Runtime_BINARY_DIR}")

add_fabric_lib(
  NAME ReliableCollectionRuntime
  OUTDIR ${CMAKE_CURRENT_BINARY_DIR}
  DLLDIR ${ServiceFabric_Runtime_BINARY_DIR}
)

# generate winmd. defer generation task in .metadata csproj.
find_program (
    dotnet_exe
    NAMES dotnet.exe
    REQUIRED
)

set(_out_winmds
  Microsoft.ServiceFabric.ReliableCollectionRuntime.winmd
  Windows.Win32.winmd
)
set(_out_winmd_paths "")
foreach(_out_winmd ${_out_winmds})
list(APPEND _out_winmd_paths ${CMAKE_CURRENT_SOURCE_DIR}/.windows/winmd/${_out_winmd})
endforeach()

add_custom_command(
  OUTPUT ${_out_winmd_paths}
  COMMAND ${dotnet_exe} build
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/.metadata
  VERBATIM
)

add_custom_target(generate_winmd
  DEPENDS ${_out_winmd_paths}
  ReliableCollectionRuntime # generator needs to scan support lib files
)

find_program (
    cargo_exe
    NAMES cargo.exe
    REQUIRED
)

add_custom_target(generate_rust
  COMMAND ${cargo_exe} run -p tools_api
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

add_subdirectory(crates/samples/kvstore)