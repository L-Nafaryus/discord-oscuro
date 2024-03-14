
function(copy_file FILE)
    if (EXISTS ${CMAKE_SOURCE_DIR}/${FILE})
        configure_file(${FILE} ${CMAKE_CURRENT_BINARY_DIR} COPYONLY)
    elseif (EXISTS ${CMAKE_CURRENT_BINARY_DIR}/${FILE})
        file(REMOVE ${CMAKE_CURRENT_BINARY_DIR}/${FILE})
    endif ()
endfunction()