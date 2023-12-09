// quick and dirty copy-paste dev-tools script

lines = document.querySelector('pre').innerText.split('\n');
toInt = {'one':1,'two':2,'three':3,'four':4,'five':5,'six':6,'seven':7,'eight':8,'nine':9};
lines.map(l => {
  matches = [...l.matchAll(/[0-9]|one|two|three|four|five|six|seven|eight|nine/g)]
  if (matches.length == 0) return 0;
  first = matches[0][0];
  first = first.length == 1 ? (+first) : toInt[first];
  last = matches[matches.length - 1][0];
  last = last.length == 1 ? (+last) : toInt[last];
  return first * 10 + last
}).reduce((a,b)=>a+b, 0)
