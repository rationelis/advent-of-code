const things = [];

const spelledOutToDigit = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
  six: "6",
  seven: "7",
  eight: "8",
  nine: "9",
};

let target = 0;

for (let i = 0; i < things.length; i++) {
  const charArr = things[i].split("");
  let digits = [];

  for (let j = 0; j < charArr.length; j++) {
    const value = getNumber(charArr.slice(j));
    if (value !== "") {
      digits.push(value);
    }
  }

  const value = parseInt(digits[0] + digits[digits.length - 1], 10);

  target += value;
}

function getNumber(arr) {
  let str = "";
  for (let i = 0; i < arr.length; i++) {
    str += arr[i];
    if (!isNaN(parseInt(str, 10))) {
      return parseInt(str, 10).toString();
    }
    if (spelledOutToDigit.hasOwnProperty(str)) {
      return spelledOutToDigit[str];
    }
  }
  return "";
}

console.log(target);
