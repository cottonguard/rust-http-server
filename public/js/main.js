let count = 0;

let button = document.querySelector('#button');
let counter = document.querySelector('#counter');
let fetched = document.querySelector('#fetched');

counter.textContent = count;

button.addEventListener('click', (e) => {
  ++count;
  counter.textContent = count;
})

fetch('json/sample.json')
.then(res => res.json())
.then(
  data => {
    fetched.textContent = JSON.stringify(data, null, 4);
  },
  err => {}
);
