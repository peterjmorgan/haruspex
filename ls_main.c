__int64 __fastcall main(int a1, const char **a2, char **a3)
{
  char *v3; // r14
  char *v4; // rax
  char *v5; // r12
  const char *v6; // r15
  unsigned __int8 v7; // al
  int v9; // ebx
  unsigned int v10; // eax
  int v11; // edx
  int v12; // ecx
  int v13; // r8d
  int v14; // r9d

  v3 = (char *)*a2;
  if ( !*a2 )
  {
    fwrite("A NULL argv[0] was passed through an exec system call.\n", 1u, 0x37u, stderr);
    abort();
  }
  v4 = strrchr(*a2, 47);
  v5 = v4;
  if ( v4 )
  {
    v6 = v4 + 1;
    if ( v4 + 1 - v3 > 6 && !strncmp(v4 - 6, "/.libs/", 7u) )
    {
      v3 = (char *)v6;
      if ( !strncmp(v6, "lt-", 3u) )
      {
        v3 = v5 + 4;
        program_invocation_short_name = v5 + 4;
      }
    }
  }
  program_invocation_name = v3;
  setlocale(6, &locale);
  bindtextdomain("coreutils", "/usr/share/locale");
  textdomain("coreutils");
  status = 2;
  sub_8E00(sub_4AD0);
  qword_C1C8 = (__int64)a2;
  dword_C1C4 = a1;
  dword_C1C0 = 1;
  if ( a1 <= 1 )
    return 1;
  v7 = sub_88F0((unsigned int)(a1 - 1));
  if ( a1 != dword_C1C0 )
  {
    v9 = sub_4B60(a2[dword_C1C0]);
    v10 = (unsigned int)dcgettext(0, "extra argument %s", 5);
    sub_7410(v10, v9, v11, v12, v13, v14);
  }
  return v7 ^ 1u;
}
