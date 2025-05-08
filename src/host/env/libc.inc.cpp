// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

static int32_t strlen(Instance *instance, uint32_t string_addr) {
  if (!VALIDATE_APP_ADDR(string_addr, sizeof(char))) {
    return 0;
  }
  const char *native_string = (const char *)ADDR_APP_TO_NATIVE(string_addr);
  return std::strlen(native_string);
}

static int32_t puts(Instance *instance, uint32_t string_addr) {
  if (!VALIDATE_APP_ADDR(string_addr, sizeof(char))) {
    return 0;
  }
  const char *native_string = (const char *)ADDR_APP_TO_NATIVE(string_addr);
  return std::puts(native_string);
}

static void internal_itoa(int n, char s[], int radix) {
  static char const HEXDIGITS[0x10] = {'0', '1', '2', '3', '4', '5', '6', '7',
                                       '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};

  int i = 0, j;
  bool negative = n < 0;

  int64_t safe_n = n;
  if (negative) {
    safe_n = -safe_n;
  }

  do {
    s[i++] = HEXDIGITS[safe_n % radix];
  } while ((safe_n /= radix) > 0);

  if (negative) {
    s[i++] = '-';
  }
  s[i] = '\0';

  // reverse
  for (i = 0, j = std::strlen(s) - 1; i < j; i++, j--) {
    char c = s[i];
    s[i] = s[j];
    s[j] = c;
  }
}

// from wasm3
static int32_t printf(Instance *instance, uint32_t format, uint32_t argv) {
  if (!VALIDATE_APP_ADDR(format, sizeof(char))) {
    return 0;
  }

  const char *native_format = (const char *)ADDR_APP_TO_NATIVE(format);

  const uint32_t *native_argv = (const uint32_t *)ADDR_APP_TO_NATIVE(argv);

  size_t fmt_len = strnlen(native_format, 1024);
  if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_format, fmt_len + 1))
    return 0; // include `\0`

  FILE *file = stdout;

  int32_t length = 0;
  char ch;
  int long_ch = 0;
  while ((ch = *native_format++)) {
    if ('%' != ch) {
      putc(ch, file);
      length++;
      continue;
    } else {
      long_ch = 0;
    }
    ch = *native_format++;
    switch (ch) {
    case 'c': {
      if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(uint32_t)))
        return 0;
      char char_temp = *native_argv++;
      fputc(char_temp, file);
      length++;
      break;
    }
    case 'l':
      long_ch++;
      native_format++;
      /* Fall through */
    case 'd':
    case 'i':
    case 'x': {
      int32_t int_temp;
      if (long_ch < 2) {
        if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(uint32_t)))
          return 0;
        int_temp = *native_argv++;
      } else {
        int64_t lld;
        if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(uint64_t)))
          return 0;
        lld = *(int64_t *)native_argv;
        native_argv += 2;
        if (lld > INT32_MAX || lld < INT32_MIN) {
          // error
          return 0;
        }
        int_temp = (int32_t)lld;
      }
      char buffer[32] = {
          0,
      };
      internal_itoa(int_temp, buffer, (ch == 'x') ? 16 : 10);
      fputs(buffer, file);
      length += strnlen(buffer, sizeof(buffer));
      break;
    }
    case 'u': {
      uint32_t int_temp;
      if (long_ch < 2) {
        if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(uint32_t)))
          return 0;
        int_temp = *native_argv++;
      } else {
        uint64_t llu;
        if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(uint64_t)))
          return 0;
        llu = *(uint64_t *)native_argv;
        native_argv += 2;
        if (llu > INT32_MAX) {
          // error
          return 0;
        }
        int_temp = (uint32_t)llu;
      }
      char buffer[32] = {
          0,
      };
      internal_itoa(int_temp, buffer, 10);
      fputs(buffer, file);
      length += strnlen(buffer, sizeof(buffer));
      break;
    }
    case 'f': {
      double d;
      /* Make 8-byte aligned */
      native_argv = (uint32_t *)(((uintptr_t)native_argv + 7) & ~(uintptr_t)7);
      if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(double)))
        return 0;

      d = *(double *)(native_argv);
      native_argv += 2;

      char buf[16];
      snprintf(buf, sizeof(buf), "%f", d);

      fputs(buf, file);
      length += strnlen(buf, sizeof(buf));
      break;
    }
    case 's': {
      if (!VALIDATE_NATIVE_ADDR((uint8_t *)native_argv, sizeof(uint32_t)))
        return 0;
      const char *string_temp;
      size_t string_len;

      string_temp = (const char *)ADDR_APP_TO_NATIVE(*native_argv++);
      if (string_temp == nullptr) {
        string_temp = "(null)";
        string_len = 6;
      } else {
        string_len = strnlen(string_temp, 1024);
        if (!VALIDATE_NATIVE_ADDR((uint8_t *)string_temp, string_len + 1))
          return 0;
      }

      fwrite(string_temp, 1, string_len, file);
      length += string_len;
      break;
    }
    default:
      fputc(ch, file);
      length++;
      break;
    }
  }

  return length;
}

#define LIBC_HOST_API_LIST                                                     \
  NATIVE_FUNC_ENTRY(strlen)                                                    \
  NATIVE_FUNC_ENTRY(puts)                                                      \
  NATIVE_FUNC_ENTRY(printf)
