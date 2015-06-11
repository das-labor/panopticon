BITS 32

global _start

section .text
aaa
aad
aad 0x33
aam
aam 0x55
aas

adc al, 6
adc ax, 44
adc eax, 544
adc al, bl
adc ax, bx
adc eax, ebx
adc [0x11223344], bl
adc [0x44332211], bx
adc [eax], ebx
adc [0x001aa1], ebx

add    byte [0x90909090+eax], dl
add    [0x90909090+eax], edx
add    dl, byte [0x90909090+eax]
add    edx, [0x90909090+eax]
add    al, 0x90
add    eax, 0x90909090
push   es
pop    es
or     byte [eax+0x90909090], dl
or     [eax+0x90909090], edx
or     dl, byte [eax+0x90909090]
or     edx, [eax+0x90909090]
or     al, byte 0x90
or     eax, 0x90909090
push   cs
adc    byte [eax+0x90909090], dl
adc    [eax+0x90909090], edx
adc    dl, byte [eax+0x90909090]
adc    edx, [eax+0x90909090]
adc    al, byte 0x90
adc    eax, 0x90909090
push   ss
pop    ss
sbb    byte [0x90909090+eax], dl
sbb    [0x90909090+eax], edx
sbb    dl, byte [0x90909090+eax]
sbb    edx, [0x90909090+eax]
sbb    al, byte 0x90
sbb    eax, 0x90909090
push   ds
pop    ds
and    byte [0x90909090+eax], dl
and    [0x90909090+eax], edx
and    dl, [0x90909090+eax]
and    edx, [0x90909090+eax]
and    al, 0x90
and    eax, 0x90909090
daa
sub    [0x90909090+eax], dl
sub    [0x90909090+eax], edx
sub    dl, [0x90909090+eax]
sub    edx, [0x90909090+eax]
sub    al, 0x90
sub    eax, 0x90909090
das
xor    [0x90909090+eax], dl
xor    [0x90909090+eax], edx
xor    dl, [0x90909090+eax]
xor    edx, [0x90909090+eax]
xor    al, 0x90
xor    eax, 0x90909090
aaa
cmp    [0x90909090+eax], dl
cmp    [0x90909090+eax], edx
cmp    dl, [0x90909090+eax]
cmp    edx, [0x90909090+eax]
cmp    al, 0x90
cmp    eax, 0x90909090
aas
inc    eax
inc    ecx
inc    edx
inc    ebx
inc    esp
inc    ebp
inc    esi
inc    edi
dec    eax
dec    ecx
dec    edx
dec    ebx
dec    esp
dec    ebp
dec    esi
dec    edi
push   eax
push   ecx
push   edx
push   ebx
push   esp
push   ebp
push   esi
push   edi
pop    eax
pop    ecx
pop    edx
pop    ebx
pop    esp
pop    ebp
pop    esi
pop    edi
pusha
popa
bound  edx, [0x90909090+eax]
arpl   [0x90909090+eax], dx
push   0x90909090
imul   edx, [0x90909090+eax], 0x90909090
push   0xffffff90
imul   edx, [0x90909090+eax], 0xffffff90
insb
insd
outsb
outsd
jo     +2-0x70
jno    +2-0x70
jb     +2-0x70
jae    +2-0x70
je     +2-0x70
jne    +2-0x70
jbe    +2-0x70
ja     +2-0x70
js     +2-0x70
jns    +2-0x70
jp     +2-0x70
jnp    +2-0x70
jl     +2-0x70
jge    +2-0x70
jle    +2-0x70
jg     +2-0x70
adc    byte [0x90909090+eax], byte 0x90
adc    [0x90909090+eax], dword 0x90909090
adc    [0x90909090+eax], dword 0xffffff90
test   [0x90909090+eax], dl
test   [0x90909090+eax], edx
xchg   [0x90909090+eax], dl
xchg   [0x90909090+eax], edx
mov    [0x90909090+eax], dl
mov    [0x90909090+eax], edx
mov    dl, [0x90909090+eax]
mov    edx, [0x90909090+eax]
mov    word [0x90909090+eax], ss
lea    edx, [0x90909090+eax]
mov    ss, word [0x90909090+eax]
pop    dword [0x90909090+eax]
xchg   eax, eax
xchg   ecx, eax
xchg   edx, eax
xchg   ebx, eax
xchg   esp, eax
xchg   ebp, eax
xchg   esi, eax
xchg   edi, eax
cwde
cdq
call   0x9090:0x90909090
;fwait
;pushf
;popf
sahf
lahf
mov    al, byte [0x90909090]
mov    eax, dword [0x90909090]
mov    [0x90909090], al
mov    [0x90909090], eax
movsb
movsd
cmpsb
cmpsd
test   al, 0x90
test   eax, 0x90909090
stosb
stosd
lodsb
lodsd
scasb
scasd
mov    al, 0x90
mov    cl, 0x90
mov    dl, 0x90
mov    bl, 0x90
mov    ah, 0x90
mov    ch, 0x90
mov    dh, 0x90
mov    bh, 0x90
mov    eax, 0x90909090
mov    ecx, 0x90909090
mov    edx, 0x90909090
mov    ebx, 0x90909090
mov    esp, 0x90909090
mov    ebp, 0x90909090
mov    esi, 0x90909090
mov    edi, 0x90909090
rcl    byte [0x90909090+eax], 0x90
rcl    dword [0x90909090+eax], 0x90
ret    0x9090
ret
les    edx, [0x90909090+eax]
lds    edx, [0x90909090+eax]
mov    byte [0x90909090+eax], 0x90
mov    dword [0x90909090+eax], 0x90909090
enter  0x9090, 0x90
leave
retf   0x9090
retf
local ret  0x9090
ret local
int3
int    0x90
into
iret
rcl    byte [0x90909090+eax], 0x90
rcl    dword [0x90909090+eax], 0x90
rcl    byte [0x90909090+eax], cl
rcl    dword [0x90909090+eax], cl
aam    0x90
aad    0x90
;xlat   byte ds:[ebx]
;fcom   [dword 0x90909090+eax]
;fst    [dword 0x90909090+eax]
;ficom  [dword 0x90909090+eax]
;fist   [dword 0x90909090+eax]
;fcom   qword [0x90909090+eax]
;fst    qword [0x90909090+eax]
;ficom  word [0x90909090+eax]
;fist   word [0x90909090+eax]
loopne +2-0x70
loope  +2-0x70
loop   +2-0x70
jecxz  +2-0x70
in     al, 0x90
in     eax, 0x90
out    0x90, al
out    0x90, eax
call   +5+0x90909090
jmp    +5+0x90909090
jmp    0x9090:0x90909090
jmp    +2-0x70
in     al, dx
in     eax, dx
out    dx, al
out    dx, eax
hlt
cmc
not    byte [0x90909090+eax]
not    dword [0x90909090+eax]
clc
stc
cli
sti
cld
std
call   [dword 0x90909090+eax]
;lldt   [0x90909090+eax]
;lgdt   [0x90909090+eax]
lar    edx, [0x90909090+eax]
;lsl    edx, [0x90909090+eax]
;clts
;invd
;wbinvd
ud2
mov    eax, cr2
mov    eax, dr2
mov    cr2, eax
mov    dr2, eax
;mov    eax, tr2
;mov    tr2, eax
;wrmsr
;rdtsc
;rdmsr
;rdpmc
cmovo  edx, [0x90909090+eax]
cmovno edx, [0x90909090+eax]
cmovb  edx, [0x90909090+eax]
cmovae edx, [0x90909090+eax]
cmove  edx, [0x90909090+eax]
cmovne edx, [0x90909090+eax]
cmovbe edx, [0x90909090+eax]
cmova  edx, [0x90909090+eax]
cmovs  edx, [0x90909090+eax]
cmovns edx, [0x90909090+eax]
cmovp  edx, [0x90909090+eax]
;cmovnp edx, [0x90909090+eax]
cmovl  edx, [0x90909090+eax]
cmovge edx, [0x90909090+eax]
cmovle edx, [0x90909090+eax]
cmovg  edx, [0x90909090+eax]
;punpcklbw mm2, [0x90909090+eax]
;punpcklwd mm2, [0x90909090+eax]
;punpckldq mm2, [0x90909090+eax]
;packsswb mm2, [0x90909090+eax]
;pcmpgtb mm2, [0x90909090+eax]
;pcmpgtw mm2, [0x90909090+eax]
;pcmpgtd mm2, [0x90909090+eax]
;packuswb mm2, [0x90909090+eax]
;punpckhbw mm2, [0x90909090+eax]
;punpckhwd mm2, [0x90909090+eax]
;punpckhdq mm2, [0x90909090+eax]
;packssdw mm2, [0x90909090+eax]
;movd   mm2, [0x90909090+eax]
;movq   mm2, [0x90909090+eax]
;psrlw  mm0, 0x90
;psrld  mm0, 0x90
;psrlq  mm0, 0x90
;pcmpeqb mm2, [0x90909090+eax]
;pcmpeqw mm2, [0x90909090+eax]
;pcmpeqd mm2, [0x90909090+eax]
;emms
;movd   [0x90909090+eax], mm2
;movq   [0x90909090+eax], mm2
jo     +6+0x90909090
jno    +6+0x90909090
jb     +6+0x90909090
jae    +6+0x90909090
je     +6+0x90909090
jne    +6+0x90909090
jbe    +6+0x90909090
ja     +6+0x90909090
js     +6+0x90909090
jns    +6+0x90909090
jp     +6+0x90909090
;jnp    .+6+0x90909090
jl     +6+0x90909090
jge    +6+0x90909090
jle    +6+0x90909090
jg     +6+0x90909090
seto   [0x90909090+eax]
setno  [0x90909090+eax]
setb   [0x90909090+eax]
setae  [0x90909090+eax]
sete   [0x90909090+eax]
setne  [0x90909090+eax]
setbe  [0x90909090+eax]
seta   [0x90909090+eax]
sets   [0x90909090+eax]
setns  [0x90909090+eax]
setp   [0x90909090+eax]
;setnp  [0x90909090+eax]
setl   [0x90909090+eax]
setge  [0x90909090+eax]
setle  [0x90909090+eax]
setg   [0x90909090+eax]
push   fs
pop    fs
cpuid
bt     [0x90909090+eax], edx
shld   [0x90909090+eax], edx, 0x90
shld   [0x90909090+eax], edx, cl
push   gs
pop    gs
;rsm
bts    [0x90909090+eax], edx
shrd   [0x90909090+eax], edx, 0x90
shrd   [0x90909090+eax], edx, cl
imul   edx, [0x90909090+eax]
cmpxchg [0x90909090+eax], dl
cmpxchg [0x90909090+eax], edx
lss    edx, [0x90909090+eax]
btr    [0x90909090+eax], edx
lfs    edx, [0x90909090+eax]
lgs    edx, [0x90909090+eax]
movzx  edx, byte [0x90909090+eax]
movzx  edx, word [0x90909090+eax]
;ud2
btc    [0x90909090+eax], edx
bsf    edx, [0x90909090+eax]
bsr    edx, [0x90909090+eax]
movsx  edx, byte [0x90909090+eax]
movsx  edx, word [0x90909090+eax]
xadd   [0x90909090+eax], dl
xadd   [0x90909090+eax], edx
bswap  eax
bswap  ecx
bswap  edx
bswap  ebx
bswap  esp
bswap  ebp
bswap  esi
bswap  edi
;psrlw  mm2, [0x90909090+eax]
;psrld  mm2, [0x90909090+eax]
;psrlq  mm2, [0x90909090+eax]
;pmullw mm2, [0x90909090+eax]
;psubusb mm2, [0x90909090+eax]
;psubusw mm2, [0x90909090+eax]
;pand   mm2, [0x90909090+eax]
;paddusb mm2, [0x90909090+eax]
;paddusw mm2, [0x90909090+eax]
;pandn  mm2, [0x90909090+eax]
;psraw  mm2, [0x90909090+eax]
;psrad  mm2, [0x90909090+eax]
;pmulhw mm2, [0x90909090+eax]
;psubsb mm2, [0x90909090+eax]
;psubsw mm2, [0x90909090+eax]
;por    mm2, [0x90909090+eax]
;paddsb mm2, [0x90909090+eax]
;paddsw mm2, [0x90909090+eax]
;pxor   mm2, [0x90909090+eax]
;psllw  mm2, [0x90909090+eax]
;pslld  mm2, [0x90909090+eax]
;psllq  mm2, [0x90909090+eax]
;pmaddwd mm2, [0x90909090+eax]
;psubb  mm2, [0x90909090+eax]
;psubw  mm2, [0x90909090+eax]
;psubd  mm2, [0x90909090+eax]
;paddb  mm2, [0x90909090+eax]
;paddw  mm2, [0x90909090+eax]
;paddd  mm2, [0x90909090+eax]
add    [0x90909090+eax], dx
add    dx, [0x90909090+eax]
add    ax, 0x9090
push word  es
pop word   es
or     [0x90909090+eax], dx
or     dx, [0x90909090+eax]
or     ax, 0x9090
push word  cs
adc    [0x90909090+eax], dx
adc    dx, [0x90909090+eax]
adc    ax, 0x9090
push word  ss
pop word   ss
sbb    [0x90909090+eax], dx
sbb    dx, [0x90909090+eax]
sbb    ax, 0x9090
push word  ds
pop word   ds
and    [0x90909090+eax], dx
and    dx, [0x90909090+eax]
and    ax, 0x9090
sub    [0x90909090+eax], dx
sub    dx, [0x90909090+eax]
sub    ax, 0x9090
xor    [0x90909090+eax], dx
xor    dx, [0x90909090+eax]
xor    ax, 0x9090
cmp    [0x90909090+eax], dx
cmp    dx, [0x90909090+eax]
cmp    ax, 0x9090
inc    ax
inc    cx
inc    dx
inc    bx
inc    sp
inc    bp
inc    si
inc    di
dec    ax
dec    cx
dec    dx
dec    bx
dec    sp
dec    bp
dec    si
dec    di
push   ax
push   cx
push   dx
push   bx
push   sp
push   bp
push   si
push   di
pop    ax
pop    cx
pop    dx
pop    bx
pop    sp
pop    bp
pop    si
pop    di
pushad
popa
bound  dx, [0x90909090+eax]
push word  0x9090
imul   dx, [0x90909090+eax], 0x9090
push word  0xff90
imul   dx, [0x90909090+eax], 0xff90
insw
outsw
adc    word [0x90909090+eax], 0x9090
adc    word [0x90909090+eax], 0xff90
test   [0x90909090+eax], dx
xchg   [0x90909090+eax], dx
mov    [0x90909090+eax], dx
mov    dx, [0x90909090+eax]
mov    word [0x90909090+eax], ss
lea    dx, [0x90909090+eax]
pop    word [0x90909090+eax]
xchg   cx, ax
xchg   dx, ax
xchg   bx, ax
xchg   sp, ax
xchg   bp, ax
xchg   si, ax
xchg   di, ax
cbw
cwd
call word  0x9090:0x9090
;pushfw
;popfw
mov    ax, [0x90909090]
mov    [0x90909090], ax
movsw
cmpsw
test   ax, 0x9090
stosw
lodsw
scasw
mov    ax, 0x9090
mov    cx, 0x9090
mov    dx, 0x9090
mov    bx, 0x9090
mov    sp, 0x9090
mov    bp, 0x9090
mov    si, 0x9090
mov    di, 0x9090
rcl    word [0x90909090+eax], 0x90
ret word   0x9090
ret
les    dx, [0x90909090+eax]
lds    dx, [0x90909090+eax]
mov    word [0x90909090+eax], 0x9090
enter 0x9090, 0x90
leave
;retfw  0x9090
;retfw
ret  0x0090
ret
iret
rcl    word [0x90909090+eax], 1
rcl    word [0x90909090+eax], cl
in     ax, 0x90
out    0x90, ax
call word  +3+0x9090
jmp word   0x9090:0x9090
in     ax, dx
out    dx, ax
not    word [0x90909090+eax]
call   word [0x90909090+eax]
lar    dx, [0x90909090+eax]
;lsl    dx, [0x90909090+eax]
cmovo  dx, [0x90909090+eax]
cmovno dx, [0x90909090+eax]
cmovb  dx, [0x90909090+eax]
cmovae dx, [0x90909090+eax]
cmove  dx, [0x90909090+eax]
cmovne dx, [0x90909090+eax]
cmovbe dx, [0x90909090+eax]
cmova  dx, [0x90909090+eax]
cmovs  dx, [0x90909090+eax]
cmovns dx, [0x90909090+eax]
cmovp  dx, [0x90909090+eax]
;cmovnp dx, [0x90909090+eax]
cmovl  dx, [0x90909090+eax]
cmovge dx, [0x90909090+eax]
cmovle dx, [0x90909090+eax]
cmovg  dx, [0x90909090+eax]
pushw  fs
popw   fs
bt     [0x90909090+eax], dx
shld   [0x90909090+eax], dx, 0x90
shld   [0x90909090+eax], dx, cl
push word  gs
pop word  gs
bts    [0x90909090+eax], dx
shrd   [0x90909090+eax], dx, 0x90
shrd   [0x90909090+eax], dx, cl
imul   dx, [0x90909090+eax]
cmpxchg [0x90909090+eax], dx
lss    dx, [0x90909090+eax]
btr    [0x90909090+eax], dx
lfs    dx, [0x90909090+eax]
lgs    dx, [0x90909090+eax]
movzx  dx, byte [0x90909090+eax]
btc    [0x90909090+eax], dx
bsf    dx, [0x90909090+eax]
bsr    dx, [0x90909090+eax]
movsx  dx, byte [0x90909090+eax]
xadd   [0x90909090+eax], dx

gs_foo:
ret

short_foo:
ret

bar:
call	gs_foo
call	short_foo
ok_till_here:
;fstp   QWORD PTR [eax+edx*8]
mov	byte [esi+edx], al
mov	byte [edx+esi], al
mov	byte [edx*2+esi], al
mov	byte [esi+edx*2], al
jmp	short rot5
insb
xadd   [0x90909090+eax], dx
and    eax, -8
rot5:
mov    eax, dword [esi+4+ecx*8]
insb
or     al, 0x90
or     eax, 0x90909090
push   cs
mov	    eax, [ebx*2]
adc    byte [eax*4+0x90909090], dl
das
jmp    0x9090:0x90909090
movsw
jo     +2-0x70

l1:
jne	l1
add	edi, dword [ebx+8*eax]
;movd	mm0, dword [ebx+8*eax+4]
add	edi, dword [ebx+8*ecx+((4095+1)*8)]
;movd	mm1, dword [ebx+8*ecx+((4095+1)*8)+4]
;movd	mm2, dword [ebx+8*eax+(2*(4095+1)*8)+4]
add	edi, dword [ebx+8*eax+(2*(4095+1)*8)]
mov	ax,  word [ebx+2*eax]
mov	cx,  word [ebx+2*ecx+((4095+1)*2)]
mov	ax,  word [ebx+2*eax+(2*(4095+1)*2)]
jmp 	eax
jmp	[eax]
jmp	[bar]
jmp	bar

; Check arithmetic operators
mov	eax,(( 17 ) + 1)
and	eax,~(1 << ( 18 ))
and	eax,0xFFFBFFFF
mov	al, (( 0x4711  ) & 0xff)
mov	al, 0x11
mov	bl, ((( 0x4711  ) >> 8) & 0xff)
mov	bl, 0x47

shrd   eax, edx, cl
shld   eax, edx, cl

;fadd
;fadd	st(3)
;fadd	st,st(3)
;fadd	st(3),st
;fadd   DWORD PTR [ebx]
;fadd   QWORD PTR [ebx]
;faddp
;faddp	st(3)
;faddp	st(3),st
;fdiv
;fdiv   st(3)
;fdiv   st,st(3)
;fdiv   st(3),st
;fdiv   DWORD PTR [ebx]
;fdiv   QWORD PTR [ebx]
;fdivp
;fdivp  st(3)
;fdivp  st(3),st
;fdivp  st,st(3)
;fdivr
;fdivr  st(3)
;fdivr  st,st(3)
;fdivr  st(3),st
;fdivr  DWORD PTR [ebx]
;fdivr  QWORD PTR [ebx]
;fdivrp
;fdivrp st(3)
;fdivrp st(3),st
;fdivrp st,st(3)
;fmul
;fmul	st(3)
;fmul	st,st(3)
;fmul	st(3),st
;fmul   DWORD PTR [ebx]
;fmul   QWORD PTR [ebx]
;fmulp
;fmulp	st(3)
;fmulp	st(3),st
;fsub
;fsubr
;fsub   st(3)
;fsub   st,st(3)
;fsub   st(3),st
;fsub   DWORD PTR [ebx]
;fsub   QWORD PTR [ebx]
;fsubp
;fsubp  st(3)
;fsubp  st,st(3)
;fsubp  st(3),st
;fsubr  st(3)
;fsubr  st,st(3)
;fsubr  st(3),st
;fsubr  DWORD PTR [ebx]
;fsubr  QWORD PTR [ebx]
;fsubrp
;fsubrp st(3)
;fsubrp st(3),st
;fsubrp st,st(3)

;fidivr  word [ebx]
;fidivr  dword [ebx]

cmovpe  edx, [0x90909090+eax]
cmovpo edx, [0x90909090+eax]
cmovpe  dx, [0x90909090+eax]
cmovpo dx, [0x90909090+eax]
