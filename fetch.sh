#!/bin/sh

curl -O https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/jcc-list.txt
curl -O https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/jcg-list.txt
curl -O https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/ku-list.txt

mkdir -p en
pushd en
curl -O http://www.jarl.or.jp/English/4_Library/A-4-5_jcc-jcg/jcc-list.txt
curl -O http://www.jarl.or.jp/English/4_Library/A-4-5_jcc-jcg/jcg-list.txt
curl -O http://www.jarl.or.jp/English/4_Library/A-4-5_jcc-jcg/KU_Data-eng.xls
popd
