.intel_syntax noprefix
adc al, 6
adc ax, 44
adc eax, 544
adc rax, 65535674
adc rax, [65535674]
adc al, bl
adc ax, bx
adc eax, ebx
adc rax, rbx
adc [0x11223344], bl
adc [0x44332211], bx
lock adc [eax], ebx
adc [0x001aa1], ebx
