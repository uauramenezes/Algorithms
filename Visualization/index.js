let i = 0;
let j = 1;
let x = 0;
let el = 50;
let array = [];
let minVal = 0;
let current = 0;
let done = false;
let play = false;

function reset() {
  array = [];
  el = 50;
  i = 0;
  j = 1;
  x = 0;
  minVal = 0;
  current = 0;
  done = false;
  play = false;
  randomArray();
}

let playBtn = document.querySelector('#play-btn');
playBtn.addEventListener('click', () => {
  if (done) {
    done = false;
    reset();
  };

  play = play ? false : true;
  playBtn.textContent = play ? 'Pause' : 'Play';
})

function setup() {
  createCanvas(700, 400);
  randomArray();
}

function randomArray() {
  array = [];
  for (let i = 0; i < el; i++) {
    array.push(random(1, height))
  }
}

function draw() {
  background(11);
  drawArray();

  if (play) speedAnimation();

  if (i>= array.length) {
    done = true;
    play = false;
    playBtn.textContent = 'Play'
  }
}

function speedAnimation() {
  let sliderValue = document.querySelector('#slider').value;
  if (sliderValue < 0) {
    if (x % -sliderValue === 0) {
      selectionSort();
    }
    x++;
  } else {
    for (let s = 0; s <= sliderValue; s++) {
      selectionSort();
    }
  }
}

function selectionSort() {
  if (j === i + 1) minVal = i;
  if (j<array.length) {
    if (array[j] < array[minVal]) minVal = j;
    current = j;
    j++;
  } else {
    swap(i, minVal)
    i++;
    j=i+1;
  }
}

function drawArray() {
  for (let c = 0; c < el; c++) {
    let wRect = width/el;
    let xPos = c * wRect;

    if ((minVal == c && minVal != 0 && !done) || (current == c && c !== 0 && !done)) {
      fill(0, 255, 0);
    } else {
      fill(0, 0, 255);
    }

    rect(xPos, height-array[c], wRect, array[c]);
  }
}

function swap(a, b) {
  let temp = array[a];
  array[a] = array[b];
  array[b] = temp;
}
