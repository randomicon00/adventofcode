fn main() {  
  
  let input = "29 PQJGK => 4 SRPZP
6 XKJP, 4 LDSMT => 6 ZVSMJ
9 CGCPW, 2 CFVS => 7 DWZH
1 VQWZC, 1 FRTJG => 1 MGLKL
3 GCTBC, 4 RGKB => 4 RLZR
2 LGLCP => 9 GCTBC
3 DFPW, 12 FRTJG => 2 XVSRT
1 VBXFG => 3 VQWZC
1 GCTBC, 1 BVHM => 8 DGMZB
7 DFPW, 3 JQNL => 7 FRTJG
6 BHQN, 4 VXNP => 5 HBLQT
1 XNBR, 27 FTBNQ, 2 RGKB => 7 JZPMK
6 HKMV, 4 JHDHS, 11 NMSKF => 9 STCX
129 ORE => 5 NVXTP
13 BVHM, 5 XNBR => 3 LKXML
3 SBPM, 4 LDSMT, 13 GPBG => 6 HXFCJ
1 XVSRT, 1 JHDHS => 4 FTBNQ
6 LKXML, 6 HRLWP => 5 PSJK
5 HRLWP, 19 PDHVG => 1 VRQD
3 FTBNQ, 1 QLKTZ => 7 SBPM
2 VXNP => 3 XKJP
4 SRPZP, 7 XVSRT => 8 LMVF
2 GPBG, 8 DWZH, 3 JTCHR, 10 RLZR, 1 CFVS, 20 BFCZ => 2 WZSTV
130 ORE => 9 JQNL
100 ORE => 4 VBXFG
4 XNBR => 8 RDSHN
2 CDBTL, 2 XNBR, 8 QLKTZ => 6 CGCPW
7 CGCPW => 6 BFCZ
7 FTBNQ, 7 VXNP => 1 BVHM
1 HXFCJ, 15 CSXD => 1 DFXPB
1 MCRW, 6 MGLKL, 1 HBLQT => 8 SWRV
19 BZQGL, 10 NMSKF, 20 WZSTV, 15 LVGB, 26 FTBNQ, 45 DWZH, 2 FJWVP, 56 JZPMK => 1 FUEL
12 JTCHR => 4 CDBTL
1 MGLKL => 6 PQJGK
1 NVXTP => 1 LDSMT
3 SWRV, 1 LGLCP => 2 GHVJ
4 DGMZB, 11 HXFCJ, 2 RLZR => 4 SHTB
5 GHVJ, 1 RGKB, 1 GCTBC => 6 HKMV
1 SRPZP => 9 XNBR
1 ZVSMJ => 2 JHDHS
9 SWRV => 5 NMSKF
1 NVXTP => 3 XKBS
7 BHQN => 2 GPBG
21 NMSKF, 12 FTBNQ, 12 SBPM => 3 CMXK
2 GPBG, 12 ZVSMJ, 2 PDHVG => 4 LGLCP
158 ORE => 8 DFPW
3 BVHM, 1 HXFCJ, 5 CGCPW, 5 BFCZ, 6 VRQD, 3 LDSMT => 7 LVGB
1 XVSRT, 1 XKJP => 8 PDHVG
3 VRQD, 16 SHTB, 5 SBPM => 9 BZQGL
1 BVHM, 3 HKMV => 4 CFVS
13 JQNL => 7 VXNP
1 XKJP, 6 XVSRT => 7 MCRW
15 NVXTP, 19 XKBS => 4 BHQN
8 JHDHS, 5 CMXK, 2 GPBG => 8 CSXD
1 JZBR, 13 LKXML, 1 HKMV, 9 DFXPB, 3 XKBS, 2 PSJK, 2 LMVF, 15 HRLWP => 7 FJWVP
1 CGCPW, 3 RDSHN => 8 JZBR
24 PQJGK => 5 JTCHR
1 XVSRT, 5 LDSMT => 6 QLKTZ
17 GPBG => 7 RGKB
4 STCX => 1 HRLWP";
  
  for (num, line) in input.lines().enumerate() {
    println!("Day 14 input data:  
      Line number: {} - 
      Line content: {}", num, line);
  }
  println!("Solution is: {}", input);
}

//Tests for the code problem number 14
#[cfg(test)]
mod tests {
     
    //use super::*;
    
    #[test]
    fn test_true() {
        assert!(true)
    }
    
    #[test]
    fn test_equal() {
        assert_eq!(true, true);
    }
}
