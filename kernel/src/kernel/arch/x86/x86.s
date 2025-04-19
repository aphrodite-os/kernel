.intel_syntax noprefix

.global reloadSegments

reloadSegments:
   mov   ax, 0x10
   mov   ds, ax
   mov   es, ax
   mov   fs, ax
   mov   gs, ax
   mov   ss, ax
   xchg  bx, bx
   call get_retaddr_ppro
   add eax, 7
   pushd 0x8
   push eax
   retf

.reload_cs:
   ret

get_retaddr_ppro:
   mov  eax, [esp]
   ret