BITS 64
SECTION .text

;aaa
;aad
;aad 0x33
;aam
;aam 0x55
;aas
;adc al, 6
;adc ax, WORD 44
;adc eax, 544
;adc rax, 65535674
;adc rax, 65535674
;adc al, bl
;adc ax, bx
;adc eax, ebx
;adc rax, rbx
;adc [0x11223344], bl
;adc [0x44332211], bx
;adc [eax], ebx
adc [0x001aa1], rbx
