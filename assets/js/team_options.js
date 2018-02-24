
var number = getURLParameter("number");

function getURLParameter(sParam) {
    var sPageURL = window.location.search.substring(1);
    var sURLVariables = sPageURL.split('&');
    for (var i = 0; i < sURLVariables.length; i++) {
        var sParameterName = sURLVariables[i].split('=');
        if (sParameterName[0] == sParam) {
            return sParameterName[1];
        }
    }

    return "";
}

function init(){
    var title = "Team Options - Team #" + number;

    document.getElementById("title").innerHTML = title;
    document.title = title;
    
    httpGetAsync("api/get/teamName/"+number, function(responseText){
        document.getElementById("teamName").innerHTML = responseText;
    });
}

function removeTeam(){
    httpDeleteAsync("api/delete/team/"+number, function(responseText){
        document.getElementById("statusBox").innerHTML = responseText;
    });
}


function httpDeleteAsync(theUrl, callback) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            callback(xmlHttp.responseText);
    }
    xmlHttp.open("DELETE", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
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