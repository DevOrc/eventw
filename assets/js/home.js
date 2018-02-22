
function init() {
    httpGetAsync("/api/get/eventName", function (responseText) {
        var eventNameLabel = document.getElementById("eventName");
        eventNameLabel.innerHTML = "Event Wizard <br />" + responseText;
    });
}

function httpGetAsync(theUrl, callback) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            callback(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}

function addTeam() {
    console.log("Test");
    var myWindow = window.open("add_teams.html", "MsgWindow", "width=500,height=400");
}