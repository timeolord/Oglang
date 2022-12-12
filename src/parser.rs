use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;

//Consonants
//b, c, ç, d, f, g, ğ, h, j, k, l, m, n, p, r, s, ş, t, v, y, z

//Vowels
//a, e, ı, i, o, ö, u, ü

//Syllable
//Taken from https://www.researchgate.net/figure/The-structure-of-Turkish-syllables_tbl1_224091517
//V
//VC
//CVC
//VCC
//CCV
//CVCC

//Nouns
// Any combination of syllables