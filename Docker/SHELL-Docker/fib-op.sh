a=0
b=1
while true
do
  let "a += b"
  echo $a
  let "b += a"
  echo $b
done
