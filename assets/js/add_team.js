function onAddTeam(){
    var teamName = document.getElementById("teamName");
    var teamNumber = document.getElementById("teamNumber");
    
    if(teamNumber.value && teamName.value){
        httpPostAsync("/api/post/createTeam/" + teamName.value + "/" + teamNumber.value, onPostRecieved)  

        teamName.value = "";
        teamNumber.value = "";
    }else{
        statusBox.innerHTML = "Please Fill in Form!";
        statusBox.style = "color:orange;"
        return;
    }
} 

function onPostRecieved(response){
    var statusBox = document.getElementById("statusBox");
    
    if(response.status == 200){
        statusBox.innerHTML = "Created Team, " + response.responseText;
        statusBox.style = "color:green;"
    }else{
        statusBox.style = "color:red;"
        statusBox.innerHTML = "Error: " + response.status;
    }

}

function httpPostAsync(theUrl, callback) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        callback(xmlHttp);
        if (xmlHttp.readyState != 4 && xmlHttp.status != 200){
            console.log("Recieved HttpRequest Error: " + xmlHttp.status);
        }
    }
    xmlHttp.open("POST", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}
