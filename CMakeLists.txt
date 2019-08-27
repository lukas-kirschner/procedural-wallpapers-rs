cmake_minimum_required(VERSION 3.6)
project(procedural-wallpapers)
set(CMAKE_CXX_STANDARD 17)

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

foreach(KIND ${IMAGEKINDS})
    add_executable(${KIND} ${COMMON_SOURCE_FILES} src/${KIND}.c)
    target_link_libraries(${KIND} m)
    target_compile_definitions(${KIND} PRIVATE PRGNAME=${KIND})
endforeach(KIND)
#add_executable(clouds ${COMMON_SOURCE_FILES} src/clouds.c)
#target_link_libraries(clouds m)
#add_executable(fern ${COMMON_SOURCE_FILES} src/fern.c)
#target_link_libraries(fern m)
#set_property(TARGET procedural-wallpapers PROPERTY C_STANDARD 11)
