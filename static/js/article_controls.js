var scrollPosition = 0;
var articleDiv = null;
var offsetNotScroll = null;
var darkTheme = false;
var zoom = 1.0;
var zoomStep = 0.1;

window.onload = function() {
    articleDiv = document.getElementById("article");
    offsetNotScroll = Math.round(articleDiv.offsetHeight * 0.1);
};


function scrollDown() {
    scrollPosition = Math.min(scrollPosition + articleDiv.offsetHeight - offsetNotScroll, articleDiv.scrollHeight);
    articleDiv.scroll(0, scrollPosition);
}

function scrollUp() {
    scrollPosition = Math.max(scrollPosition - articleDiv.offsetHeight + offsetNotScroll, 0);
    articleDiv.scroll(0, scrollPosition);
}

function scrollToTop() {
    scrollPosition = 0;
    articleDiv.scroll(0, scrollPosition);
}

function scrollToBottom() {
    scrollPosition = articleDiv.scrollHeight - articleDiv.offsetHeight;
    articleDiv.scroll(0, scrollPosition);
}

function zoomIn() {
    zoom += zoomStep;
    articleDiv.style.zoom = zoom;
}

function zoomOut() {
    zoom -= zoomStep;
    articleDiv.style.zoom = zoom;
}

function toggleTheme() {
    document
      .querySelectorAll('link[rel=stylesheet].alternate')
      .forEach(function (node) { node.disabled = !node.disabled; });

    let toggleThemeButton = document.getElementById("toggleThemeButton");

    if (darkTheme) {
        toggleThemeButton.classList.remove("fa-moon-o");
        toggleThemeButton.classList.add("fa-sun-o");
    } else {
        toggleThemeButton.classList.remove("fa-sun-o");
        toggleThemeButton.classList.add("fa-moon-o");
    }

    darkTheme = !darkTheme;
}
