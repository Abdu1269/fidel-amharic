# Keyboard Layouts Specification — Fidel Amharic

Fidel Amharic ships with declarative phonetic profiles that map standard US-QWERTY key sequences into standard Unicode Ethiopic characters.

## 1. Power Geez Phonetic-Compatible (Unicode)

This layout reproduces the traditional Power Geez (Phonetic) key sequences while outputting pure, unadulterated Unicode Ethiopic script (`U+1200..U+137F`).

### 1.1 Vowel Order Sequences

In Ethiopic Saba Fidel, each base consonant has seven standard vowel orders. In Power Geez Phonetic:

| Order | Traditional Name | Vowel Sound | Key Sequence Example (H family) | Unicode Character |
|:---:|:---:|:---:|:---:|:---:|
| **1st** | ግዕዝ (Ge'ez) | ä | `h` | **ሀ** (`U+1200`) |
| **2nd** | ካዕብ (Ka'eb) | u | `hu` | **ሁ** (`U+1201`) |
| **3rd** | ሣልስ (Sales) | i | `hi` | **ሂ** (`U+1202`) |
| **4th** | ራብዕ (Rabe) | a | `ha` | **ሃ** (`U+1203`) |
| **5th** | ኃምስ (Hames) | é / e | `hy` | **ሄ** (`U+1204`) |
| **6th** | ሳድስ (Sades) | ə / silent | `he` | **ህ** (`U+1205`) |
| **7th** | ሳብዕ (Sabe) | o | `ho` | **ሆ** (`U+1206`) |

### 1.2 Base Consonants Map

| Key | Base Syllable | Family Name | Key | Base Syllable | Family Name |
|:---:|:---:|:---:|:---:|:---:|:---:|
| `h` | ሀ | Hä | `w` | ወ | Wä |
| `l` | ለ | Lä | `X` | ዐ | ʿA |
| `H` | ሐ | Ḥä | `z` | ዘ | Zä |
| `m` | መ | Mä | `Z` / `zh` | ዠ | Žä |
| `J` | ሠ | Śä | `Y` | የ | Yä |
| `r` | ረ | Rä | `d` | ደ | Dä |
| `s` | ሰ | Sä | `D` | ዸ | Ḋä |
| `S` / `sh` | ሸ | Šä | `j` | ጀ | Jä |
| `q` | ቀ | Qä | `g` | ገ | Gä |
| `Q` | ቐ | Q̇ä | `G` | ጘ | G̃ä |
| `b` | በ | Bä | `T` | ጠ | Ṭä |
| `v` | ቨ | Vä | `C` / `ch` | ጨ | Č̣ä |
| `t` | ተ | Tä | `P` | ጰ | P̣ä |
| `c` / `ch` | ቸ | Čä | `F` / `ts` | ጸ | Ṣä |
| `L` | ኀ | Ḫä | `V` | ፀ | Ṣ́ä |
| `n` | ነ | Nä | `f` | ፈ | Fä |
| `N` / `ny` | ኘ | Ñä | `p` | ፐ | Pä |
| `x` | አ | ʾA | `k` | ከ | Kä |
| `K` / `kh` | ኸ | Ḫä | | | |

### 1.3 Alternative Characters via `[`

| Sequence | Character | Name |
|:---:|:---:|:---:|
| `h[` | **ኀ** | Ḫä |
| `H[` | **ኸ** | Ḫä |
| `s[` | **ሠ** | Śä |
| `t[` | **ጸ** | Ṣä |
| `T[` | **ፀ** | Ṣ́ä |

### 1.4 Ethiopic Punctuation

| Key Sequence | Ethiopic Character | Unicode | Name |
|:---:|:---:|:---:|:---:|
| `:` | **፡** | `U+1361` | Word separator space |
| `::` | **፨** | `U+1368` | Paragraph separator |
| `.` | **።** | `U+1362` | Full stop (Period) |
| `,` | **፥** | `U+1365` | Colon |
| `,,` | **፣** | `U+1363` | Comma |
| `;` | **፤** | `U+1364` | Semicolon |
| `:-` | **፦** | `U+1366` | Preface colon |
| `:.` | **፧** | `U+1367` | Question mark |

### 1.5 Ethiopic Numerals

Ethiopic digits are triggered using the digit followed by `*`:

| Keys | Numeral | Keys | Numeral |
|:---:|:---:|:---:|:---:|
| `1*` | **፩** (1) | `10*` | **፲** (10) |
| `2*` | **፪** (2) | `20*` | **፳** (20) |
| `3*` | **፫** (3) | `30*` | **፴** (30) |
| `4*` | **፬** (4) | `40*` | **፵** (40) |
| `5*` | **፭** (5) | `50*` | **፶** (50) |
| `6*` | **፮** (6) | `60*` | **፷** (60) |
| `7*` | **፯** (7) | `70*` | **፸** (70) |
| `8*` | **፰** (8) | `80*` | **፹** (80) |
| `9*` | **፱** (9) | `90*` | **፺** (90) |
| `100*` | **፻** (100) | `10000*` | **፼** (10000) |
