"use strict";

// Current question.
var q;
var questionNumber = document.getElementById("question-number");
var current = document.getElementById("current");

// Code block.
var accordion = document.getElementById("accordion");
var code = document.getElementById("code");

var form = document.getElementById("form");

// Radio buttons.
var radioUndefined = document.getElementById("radio-undefined");
var radioError = document.getElementById("radio-error");
var radioOutput = document.getElementById("radio-output");

// Input area where answer is typed in.
var textOutput = document.getElementById("text-output");

// Alert containing explanation.
var explanationAlert = document.getElementById("explanation-alert");
var explanationCorrect = document.getElementById("explanation-correct");
var explanationContent = document.getElementById("explanation-content");

// Navigation buttons.
var nav = document.getElementById("nav");
var buttonNext = document.getElementById("button-next");
var buttonSubmit = document.getElementById("button-submit");
var buttonHint = document.getElementById("button-hint");
var buttonSkip = document.getElementById("button-skip");
var buttonReveal = document.getElementById("button-reveal");
var buttonPlayground = document.getElementById("button-playground");

// Alert containing hint.
var hintAlert = document.getElementById("hint-alert");
var hintContent = document.getElementById("hint-content");

// Progress.
var state = {};
var incorrect = document.getElementById("incorrect");
var answered = document.getElementById("answered");
var total = document.getElementById("total");
var buttonReset = document.getElementById("reset");

function init() {
    window.onpopstate = function(event) {
        function activate() {
            if (event.state) {
                q = event.state.question;
            } else {
                initQuestion();
            }
            activateQuestion();
        }

        accordion.classList.remove("show");
        window.setTimeout(activate, 400);
    };
    form.addEventListener("submit", checkAnswer);
    buttonNext.onclick = nextQuestion;
    buttonSubmit.onclick = checkAnswer;
    buttonHint.onclick = doHint;
    buttonSkip.onclick = nextQuestion;
    buttonReveal.onclick = doReveal;
    textOutput.onclick = function() {
        radioOutput.checked = true;
    };
    textOutput.oninput = function() {
        hide(incorrect);
        radioOutput.checked = true;
    };
    radioUndefined.onchange = radioError.onchange = function() {
        hide(incorrect);
    };
    radioOutput.onchange = function() {
        hide(incorrect);
        textOutput.select();
    };
    total.innerHTML = countQuestions();
    loadState();
    updateProgress();
    initQuestion();
    activateQuestion();
}

function initQuestion() {
    var history = window.history.state;
    if (history && typeof history.question === "number") {
        q = history.question;
        return;
    }

    var path = window.location.pathname;
    var pathMatch = /^\/rust-quiz\/([0-9]+)\/?$/g.exec(path);
    if (pathMatch !== null) {
        var key = pathMatch[1];
        var number = parseInt(key, 10);
        if (!isNaN(number) && key in questions) {
            q = number;
        }
    }

    if (typeof q === "undefined") {
        pickRandomQuestion();
    }

    try {
        setTitle();
        var path = "/rust-quiz/" + q;
        window.history.replaceState({question: q}, document.title, path);
    }
    catch(e) {
    }
}

function activateQuestion() {
    current.innerHTML = q;
    questionNumber.title = "Difficulty:  " + difficultyStars();
    show(questionNumber);
    code.innerHTML = "";
    code.appendChild(document.createTextNode(questions[q].code.trim()));
    hljs.highlightBlock(code);
    hide(buttonPlayground);
    hide(explanationAlert);
    hide(incorrect);
    hide(hintAlert);
    buttonSkip.blur();
    radioUndefined.checked = false;
    radioError.checked = false;
    radioOutput.checked = false;
    textOutput.value = "";
    buttonHint.disabled = false;
    show(nav);
    setTitle();
    accordion.classList.add("show");
}

function difficultyStars() {
    var filled = questions[q].difficulty;
    var empty = 3 - filled;
    return "★".repeat(filled) + "☆".repeat(empty);
}

function nextQuestion() {
    function activate() {
        loadState();
        pickRandomQuestion();
        activateQuestion();

        try {
            var path = "/rust-quiz/" + q;
            window.history.pushState({question: q}, document.title, path);
        }
        catch(e) {
        }
    }

    accordion.classList.remove("show");
    window.setTimeout(activate, 400);
}

function checkAnswer(e) {
    e.preventDefault();

    var correct;
    if (radioUndefined.checked) {
        correct = questions[q].answer === "undefined";
    } else if (radioError.checked) {
        correct = questions[q].answer === "error";
    } else if (radioOutput.checked) {
        correct = questions[q].answer === textOutput.value.trim();
    } else {
        buttonSubmit.blur();
        return false;
    }

    if (correct) {
        show(explanationCorrect);
        recordCorrect();
        showExplanation();
    } else {
        show(incorrect);
        buttonSubmit.blur();
    }

    return false;
}

function doHint() {
    hintContent.innerHTML = questions[q].hint.trim();
    show(hintAlert);
    buttonHint.blur();
    buttonHint.disabled = true;
}

function doReveal() {
    hide(explanationCorrect);
    showExplanation();
}

function showExplanation() {
    hide(incorrect);
    hide(nav);
    hide(hintAlert);
    explanationContent.innerHTML = questions[q].explanation.trim();
    if (questions[q].answer === "undefined") {
        radioUndefined.checked = true;
    } else if (questions[q].answer === "error") {
        radioError.checked = true;
    } else {
        radioOutput.checked = true;
        textOutput.value = questions[q].answer;
    }
    buttonPlayground.href =
        "https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&code="
        + encodeURIComponent(questions[q].code.trim());
    show(buttonPlayground);
    show(explanationAlert);
    textOutput.blur();
}

function loadState() {
    if (storageAvailable()) {
        var json = window.localStorage.getItem("rust-quiz-answered");
        if (json) {
            state = JSON.parse(json);
        } else {
            state = {};
        }
    }
}

function saveState() {
    if (storageAvailable()) {
        var json = JSON.stringify(state);
        window.localStorage.setItem("rust-quiz-answered", json);
    }
}

function pickRandomQuestion() {
    var candidates = [];
    var unanswered = [];
    for (var i in questions) {
        var number = parseInt(i, 10);
        if (isNaN(number) || number === q) {
            continue;
        }
        candidates.push(number);
        if (!state[number]) {
            unanswered.push(number);
        }
    }

    if (unanswered.length > 0) {
        candidates = unanswered;
    }

    var rand = Math.floor(Math.random() * candidates.length);
    q = candidates[rand];
}

function setTitle() {
    document.title = "Rust Quiz #" + q;
}

function countQuestions() {
    var size = 0;
    for (var key in questions) {
        size += 1;
    }
    return size;
}

function updateProgress() {
    var count = 0;
    for (var key in questions) {
        if (state[key]) {
            count++;
        }
    }
    answered.innerHTML = count;
    if (count === countQuestions()) {
        show(contribute);
    }
    if (count > 0) {
        show(buttonReset);
    }
}

function recordCorrect() {
    loadState();
    state[q] = true;
    saveState();
    updateProgress();
}

function reset() {
    state = {};
    window.localStorage.clear();
    answered.innerHTML = 0;
    hide(contribute);
    hide(buttonReset);
}

function show(element) {
    element.classList.remove("d-none");
}

function hide(element) {
    element.classList.add("d-none");
}

function storageAvailable() {
    try {
        var x = "__storage_test__";
        window.localStorage.setItem(x, x);
        window.localStorage.removeItem(x);
        return true;
    }
    catch(e) {
        return false;
    }
}

init();
