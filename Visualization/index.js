let c;
let i;
let j;
let done;
let play;
let array;
let current;
let arraySize;
let selectedValue;

let playBtn = document.querySelector('#play-btn');
playBtn.addEventListener('click', () => {
  if (done) {
    done = false;
    initiateValues();
  };

  play = play ? false : true;
  playBtn.textContent = play ? 'Pause' : 'Play';
});

function setup() {
  createCanvas(700, 400);
  initiateValues();
}

function initiateValues() {
  i = 0;
  j = 1;
  x = 0;
  c = 0;
  array = [];
  current = 0;
  done = false;
  play = false;
  arraySize = 50;
  selectedValue = 0;
  randomArray();
}

function randomArray() {
  for (let i = 0; i < arraySize; i++) {
    array.push(random(1, height))
  }
}

function draw() {
  background(11);
  drawArray();

  if (play) playAnimation();
  if (i>= array.length) {
    done = true;
    play = false;
    playBtn.textContent = 'Play'
  }
}

function playAnimation() {
  let animationSpeed = document.querySelector('#slider').value;
  if (animationSpeed < 0) {
    if (c % -animationSpeed === 0) {
      done = selectionSort();
    }
    c++;
  } else {
    for (let n = 0; n <= animationSpeed; n++) {
      done = selectionSort();
    }
  }
}

function drawArray() {
  for (let c = 0; c < arraySize; c++) {
    let wRect = width/arraySize;
    let xPos = c * wRect;

    if ((selectedValue == c && selectedValue != 0 && !done) || (current == c && c !== 0 && !done)) {
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
  play = true;
}
