button_press = () => {
    console.log("Hello there");
};

'use strict';
window.addEventListener('load', () => {
    let form = document.getElementById('form');
    if(form) {
        form.addEventListener('submit', (e) => {
            console.log(e);
            if(form.checkValidity() === false) {
                e.preventDefault();
                e.stopPropagation();
            }
        }, false);
    }
});