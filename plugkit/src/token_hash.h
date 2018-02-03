/* ANSI-C code produced by gperf version 3.0.3 */
/* Command-line:
 * /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/gperf
 * -LANSI-C -G --output-file=plugkit/src/token_hash.h plugkit/src/token.keys  */
/* Computed positions: -k'2,4-5,16,$' */

#if !(                                                                         \
    (' ' == 32) && ('!' == 33) && ('"' == 34) && ('#' == 35) && ('%' == 37) && \
    ('&' == 38) && ('\'' == 39) && ('(' == 40) && (')' == 41) &&               \
    ('*' == 42) && ('+' == 43) && (',' == 44) && ('-' == 45) && ('.' == 46) && \
    ('/' == 47) && ('0' == 48) && ('1' == 49) && ('2' == 50) && ('3' == 51) && \
    ('4' == 52) && ('5' == 53) && ('6' == 54) && ('7' == 55) && ('8' == 56) && \
    ('9' == 57) && (':' == 58) && (';' == 59) && ('<' == 60) && ('=' == 61) && \
    ('>' == 62) && ('?' == 63) && ('A' == 65) && ('B' == 66) && ('C' == 67) && \
    ('D' == 68) && ('E' == 69) && ('F' == 70) && ('G' == 71) && ('H' == 72) && \
    ('I' == 73) && ('J' == 74) && ('K' == 75) && ('L' == 76) && ('M' == 77) && \
    ('N' == 78) && ('O' == 79) && ('P' == 80) && ('Q' == 81) && ('R' == 82) && \
    ('S' == 83) && ('T' == 84) && ('U' == 85) && ('V' == 86) && ('W' == 87) && \
    ('X' == 88) && ('Y' == 89) && ('Z' == 90) && ('[' == 91) &&                \
    ('\\' == 92) && (']' == 93) && ('^' == 94) && ('_' == 95) &&               \
    ('a' == 97) && ('b' == 98) && ('c' == 99) && ('d' == 100) &&               \
    ('e' == 101) && ('f' == 102) && ('g' == 103) && ('h' == 104) &&            \
    ('i' == 105) && ('j' == 106) && ('k' == 107) && ('l' == 108) &&            \
    ('m' == 109) && ('n' == 110) && ('o' == 111) && ('p' == 112) &&            \
    ('q' == 113) && ('r' == 114) && ('s' == 115) && ('t' == 116) &&            \
    ('u' == 117) && ('v' == 118) && ('w' == 119) && ('x' == 120) &&            \
    ('y' == 121) && ('z' == 122) && ('{' == 123) && ('|' == 124) &&            \
    ('}' == 125) && ('~' == 126))
/* The character set is not based on ISO-646.  */
#error                                                                         \
    "gperf generated tables don't work with this execution character set. Please report a bug to <bug-gnu-gperf@gnu.org>."
#endif

#define TOTAL_KEYWORDS 84
#define MIN_WORD_LENGTH 1
#define MAX_WORD_LENGTH 24
#define MIN_HASH_VALUE 1
#define MAX_HASH_VALUE 263
/* maximum key range = 263, duplicates = 0 */

#ifdef __GNUC__
__inline
#else
#ifdef __cplusplus
inline
#endif
#endif
    static unsigned int
    hash(register const char *str, register unsigned int len) {
  static unsigned short asso_values[] = {
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      5,   10,  264, 264, 264, 264, 264, 0,   264, 5,   264, 264, 264, 90,  264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 105, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 0,   264, 0,   264, 55,  264, 45,  5,   55,  15,  90,  65,
      0,   264, 80,  70,  85,  30,  5,   0,   30,  30,  30,  15,  0,   0,   264,
      65,  0,   264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264, 264,
      264};
  register unsigned int hval = len;

  switch (hval) {
  default:
    hval += asso_values[(unsigned char)str[15]];
  /*FALLTHROUGH*/
  case 15:
  case 14:
  case 13:
  case 12:
  case 11:
  case 10:
  case 9:
  case 8:
  case 7:
  case 6:
  case 5:
    hval += asso_values[(unsigned char)str[4]];
  /*FALLTHROUGH*/
  case 4:
    hval += asso_values[(unsigned char)str[3]];
  /*FALLTHROUGH*/
  case 3:
  case 2:
    hval += asso_values[(unsigned char)str[1]];
  /*FALLTHROUGH*/
  case 1:
    break;
  }
  return hval + asso_values[(unsigned char)str[len - 1]];
}

static const char *wordlist[] = {"",
                                 "_",
                                 "",
                                 "",
                                 "ipv4",
                                 "[udp]",
                                 "[ipv4]",
                                 "",
                                 "udp",
                                 "",
                                 "",
                                 "[ipv6]",
                                 "",
                                 "",
                                 "ipv6",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "[tcp]",
                                 "",
                                 "ipv4.id",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "ipv6.hopByHop",
                                 "",
                                 "",
                                 "",
                                 "ipv4.protocol.udp",
                                 "ipv4.dst",
                                 "",
                                 "",
                                 "",
                                 "ipv6.protocol.udp",
                                 "ipv6.dst",
                                 "!out-of-memory",
                                 "@ipv4:addr",
                                 "",
                                 "udp.dst",
                                 "ipv6.hopLimit",
                                 "",
                                 "@ipv6:addr",
                                 "",
                                 "",
                                 "tcp",
                                 "",
                                 "ipv4.flags",
                                 "",
                                 "ipv4.version",
                                 "eth.type.ipv4",
                                 "",
                                 "",
                                 "",
                                 "ipv6.version",
                                 "eth.type.ipv6",
                                 "ipv4.fragmentOffset",
                                 "ipv6.nextHeader",
                                 "",
                                 "",
                                 "ipv4.src",
                                 "ipv4.flags.reserved",
                                 "",
                                 "",
                                 "",
                                 "ipv6.src",
                                 "!out-of-bounds",
                                 "",
                                 "",
                                 "ipv4.protocol.tcp",
                                 "ipv4.protocol.icmp",
                                 "ipv4.type",
                                 "",
                                 "",
                                 "ipv6.protocol.tcp",
                                 "ipv6.protocol.icmp",
                                 "_.payload",
                                 "",
                                 "",
                                 "tcp.dst",
                                 "eth",
                                 "",
                                 "",
                                 "",
                                 "@nested",
                                 "ipv4.ttl",
                                 "tcp.dataOffset",
                                 "",
                                 "",
                                 "ipv6.trafficClass",
                                 "ipv4.protocol",
                                 "",
                                 "",
                                 "",
                                 "udp.src",
                                 "ipv6.protocol",
                                 "ipv6.flowLevel",
                                 "",
                                 "",
                                 "tcp.streamId",
                                 "eth.type",
                                 "",
                                 "",
                                 "_.timestamp",
                                 "ipv4.headerLength",
                                 "ipv4.checksum",
                                 "tcp.flags",
                                 "",
                                 "",
                                 "tcp.flags.ns",
                                 "tcp.flags.cwr",
                                 "",
                                 "",
                                 "",
                                 "_.index",
                                 "ipv4.protocol.igmp",
                                 "[unknown]",
                                 "",
                                 "",
                                 "tcp.seq",
                                 "ipv6.protocol.igmp",
                                 "!invalid-value",
                                 "[eth]",
                                 "",
                                 "",
                                 "@int:oct",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "tcp.src",
                                 "tcp.flags.ece",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@int:bin",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@novalue",
                                 "_.actualLength",
                                 "@date:unix",
                                 "",
                                 "",
                                 "ipv4.flags.dontFragment",
                                 "",
                                 "",
                                 "ipv4.totalLength",
                                 "udp.checksum",
                                 "@int:dec",
                                 "",
                                 "udp.length",
                                 "",
                                 "",
                                 "tcp.flags.ack",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "ipv4.flags.moreFragments",
                                 "",
                                 "",
                                 "",
                                 "tcp.flags.urg",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@int:hex",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "ipv6.payloadLength",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@flags",
                                 "tcp.ack",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@stream",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@enum",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "",
                                 "@eth:mac"};

const char *in_word_set(register const char *str, register unsigned int len) {
  if (len <= MAX_WORD_LENGTH && len >= MIN_WORD_LENGTH) {
    unsigned int key = hash(str, len);

    if (key <= MAX_HASH_VALUE) {
      register const char *s = wordlist[key];

      if (*str == *s && !strcmp(str + 1, s + 1))
        return s;
    }
  }
  return 0;
}
