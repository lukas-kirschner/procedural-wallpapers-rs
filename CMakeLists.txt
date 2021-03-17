cmake_minimum_required(VERSION 3.6)
project(procedural-wallpapers)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/bin")

include(ExternalProject)
find_package(Git REQUIRED)

ExternalProject_Add(
        cxxopts
        PREFIX ${CMAKE_CURRENT_SOURCE_DIR}/ext
        SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}/cxxopts
        GIT_REPOSITORY https://github.com/jarro2783/cxxopts.git
        GIT_TAG v2.2.1
        TIMEOUT 10
        CONFIGURE_COMMAND ""
        BUILD_COMMAND ""
        INSTALL_COMMAND ""
        LOG_DOWNLOAD ON
)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
#add_subdirectory(
#        ${CMAKE_CURRENT_SOURCE_DIR}/cxxopts
#)
include_directories(
        ${CMAKE_CURRENT_SOURCE_DIR}/cxxopts/include
)

set(IMAGEKINDS
        clouds
        fern
        flow
        islands
        landscape
        lightning
        marrowlike
        mesh
        tangles
        water
        wood
        zebra
        pixelstars
        )

set(COMMON_SOURCE_FILES
        src/lib/main.cpp
        src/lib/main.h
        src/lib/basicdrawing.cpp
        src/lib/basicdrawing.h
        src/lib/perlin.cpp
        src/lib/perlin.h
        src/lib/randomcomplex.cpp
        src/lib/randomcomplex.h
        )

add_custom_target(run_all
        )
foreach (KIND ${IMAGEKINDS})
    add_executable(${KIND} ${COMMON_SOURCE_FILES} src/${KIND}.cpp)
    target_link_libraries(${KIND} m)
    target_compile_definitions(${KIND} PRIVATE PRGNAME=${KIND})
    add_dependencies(${KIND} cxxopts)
    add_custom_target(run_${KIND}
            COMMAND ${KIND}
            DEPENDS ${KIND}
            WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
            )
    add_dependencies(run_all run_${KIND})
endforeach (KIND)