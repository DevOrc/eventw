
function init() {
    httpGetAsync("/api/get/eventName", function (responseText) {
        var eventNameLabel = document.getElementById("eventName");
        eventNameLabel.innerHTML = "Event Wizard <br />" + responseText;
    });

    httpGetAsync("/api/get/teams", loadTeamTable);
}

function refreshTeamTable() {
    httpGetAsync("/api/get/teams", loadTeamTable);
}

function loadTeamTable(responseText) {
    var table = document.getElementById("team-table");
    var html = [];
    html.push("<tr><th>Team Name</th> <th>Team Number</th><th>Options</th></tr>");

    var lines = responseText.split('\n');

    for (var i = 0; i < lines.length; i++) {
        var line = lines[i];

        if (line) {
            var teamName = line.split(",")[0];
            var teamNumber = line.split(",")[1];

            html.push(
                "<tr><td>",
                teamName,
                "</td><td>",
                teamNumber,
                "</td><td><center>",
                "<input type=\"submit\" value=\"Options\" onclick=\"openTeamOptions(",
                teamNumber,
                ")\">",
                "</center></td></tr>"
            );
        }
    }
    table.innerHTML = html.join("");
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
    window.open("add_teams.html", "MsgWindow", "width=500,height=400");
}

function openTeamOptions(number){
    window.open("team_options.html?number=" + number, "MsgWindow", "width=500,height=400");
}