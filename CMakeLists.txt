cmake_minimum_required(VERSION 3.6)
project(procedural-wallpapers)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/bin")

set (IMAGEKINDS
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
        src/lib/main.c
        src/lib/main.h
        src/lib/basicdrawing.c
        src/lib/basicdrawing.h
        src/lib/perlin.c
        src/lib/perlin.h
        src/lib/randomcomplex.c
        src/lib/randomcomplex.h
        )

add_custom_target(run_all
        )
foreach(KIND ${IMAGEKINDS})
    add_executable(${KIND} ${COMMON_SOURCE_FILES} src/${KIND}.c)
    target_link_libraries(${KIND} m)
    target_compile_definitions(${KIND} PRIVATE PRGNAME=${KIND})
    add_custom_target(run_${KIND}
            COMMAND ${KIND}
            DEPENDS ${KIND}
            WORKING_DIRECTORY ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
            )
    add_dependencies(run_all run_${KIND})
endforeach(KIND)