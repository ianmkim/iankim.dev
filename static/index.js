let sideNav =document.getElementById("mySidenav");
let theMenu = document.querySelector('.menu');
let main = document.getElementById('main');
let theOverlay = document.querySelector('.overlay');
let closeBtn = document.querySelector('.closebtn');

let theLinks = sideNav.querySelectorAll('a');

document.addEventListener('click', navigate);

function navigate(e) {
  if(e.target == theMenu) {
    sideNav.style.width = "100vw";
    document.getElementById("main").style.marginLeft = "10vw";
  theOverlay.classList.toggle('active'); 
  for(i = 0; i <= theLinks.length; i++) { theLinks[i].classList.toggle('show'); 
theLinks[i].style.transitionDelay = i / 10.2 + 's'; 
}//for loop end   
  }
  //if open
  
  else if(e.target == closeBtn || e.target == theOverlay){  sideNav.style.width = "0";
    document.getElementById("main").style.marginLeft= "0";
 theOverlay.classList.toggle('active');
for(i = 0; i <= theLinks.length; i++) { theLinks[i].classList.toggle('show'); }
  }   
}




var interv = setInterval(increaseOpacity, 100);

function increaseOpacity(){
    var background = document.getElementsByClassName("page-background")[0];
    var opacity = parseFloat(background.style.opacity)
    if(opacity >= 0.4){
        clearInterval(interv);
    }
    background.style.opacity = opacity + 0.025;
}

