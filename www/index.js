import * as wasm from "calendar";

const Calendars = {
    Gregorian: "Gregorian",
    Julian: "Julian",
    Hebrew: "Hebrew"
}

const date = wasm.js_api(Calendars.Gregorian, Calendars.Julian, 11, "September", 2022, "AD");
console.log(dateToString(date, Calendars.Julian));

const selectCal = document.getElementById("selectCal");
selectCal.addEventListener("change", calSelect);


// Converts a date object to a string
function dateToString(date, cal) {
    let output = '';

    if(date.get_is_valid() == false){
        return 'invalid date';

    }
    if (date.get_is_not_overflow() == false) {
        return 'date too early or late';
    }

    const RomanMonths = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    const HebrewMonths = ["Tishrei", "Chesvan", "Kislev", "Tevet", "Shevat", "Adar", "Nisan", "Iyar", "Sivan", "Tammuz", "Av", "Elul", "Adar I", "Adar II"];

    // Add Day:
    output += date.get_day();

    // Add Month:
    output += ' ';
    if ((cal == Calendars.Gregorian) || (cal == Calendars.Julian)) {
        // takes the month name index and converts it to the actual month name
        output += RomanMonths[date.get_month_name() - 1];
    }
    if (cal == Calendars.Hebrew) {
        // takes the month name index and converts it to the actual month name
        output += HebrewMonths[date.get_month_name() - 1];
    }

    // Add Year:
    output += ' ';
    output += date.get_year();

    // Add Era (if applicable):
    if ((cal == Calendars.Gregorian) || (cal == Calendars.Julian)) {
        output += ' '
        if (date.get_era() == true) {
            output += "AD"
        } else {
            output += "BC"
        }
    }
    return output;
}

// parses the date conversion form. Then converts the data and returns a string showing the data or an error message if the user incorrectly entered invalid or incomplete information
function parseForm() {
    const incompleteFormMsg = "You left important information blank";

    const fromCal = document.getElementById("selectCal").value;
    const toCal = document.getElementById("toCal").value;

    const day = document.getElementById("day").value;
    const month = document.getElementById("month").value;
    const year = document.getElementById("year").value;

    if(fromCal == "" || toCal == "" || day == "" || month == "" || year == ""){
        return incompleteFormMsg;
    }

    var era;
    if (fromCal == Calendars.Gregorian || fromCal == Calendars.Julian) {
        era = document.getElementById("era").value;
        if(era == ""){
            return incompleteFormMsg;
        }
    } else {
        era = "";
    }

    const date = wasm.js_api(fromCal, toCal, day, month, year, era);
    const dateString = dateToString(wasm.js_api(fromCal, toCal, day, month, year, era), toCal);

    return dateString;
}

// parses the requested date conversion and updates the page accordingly
function displayDate() {

    const dateString = parseForm();

    if(document.getElementById("displayDate") != null){
        const displayDate = document.getElementById("displayDate");
        displayDate.remove();
    }
    const displayDate = document.createElement("p");
    displayDate.id = "displayDate";
    displayDate.appendChild(document.createTextNode(dateString));
    document.getElementById("dateDiv").appendChild(displayDate);
    
}

// shows the appropriate conversion form depending on the selected input format
function calSelect() {
    console.log("Hi there, I see you're looking at the console");
    const selectElement = document.querySelector('#selectCal');
    const output = selectElement.value;
    showForm(output);
  }

// shows the date conversion form
function showForm(val) {
    const previousForm = document.getElementById("selectDate");
    if (previousForm != null) {
        previousForm.remove();
    }

    var form = document.createElement("div");
    form.id = "selectDate";

    if ((val == "Julian") || (val == "Gregorian")) {
        var eraLabel = document.createElement("label");
        eraLabel.for = "era";
        eraLabel.appendChild(document.createTextNode("Era: "));
        form.append(eraLabel);

        var era = document.createElement("select");
        era.id = "era";
        era.name = "era";

        var blankEra = document.createElement("option");
        blankEra.name = "";
        blankEra.value = "";
        era.appendChild(blankEra);

        var BC = document.createElement("option");
        BC.name = "BC";
        BC.value = "BC";
        BC.appendChild(document.createTextNode("BC"));
        era.appendChild(BC);

        var AD = document.createElement("option");
        AD.name = "AD";
        AD.value = "AD";
        AD.appendChild(document.createTextNode("AD"));
        era.appendChild(AD);

        form.appendChild(era);
    }

    var yearLabel = document.createElement("label");
    yearLabel.for = "year";
    yearLabel.appendChild(document.createTextNode("Year: "));
    form.append(yearLabel);

    var year = document.createElement("input");
    year.type = "number";
    year.min = "1";
    year.id = "year";
    year.name = "year";
    form.appendChild(year);

    var monthLabel = document.createElement("label");
    monthLabel.for = "month";
    monthLabel.appendChild(document.createTextNode("Month: "));
    form.append(monthLabel)
    
    var month = document.createElement("select");
    month.id = "month";
    month.name = "month";
    var monthArray = Array();
    monthArray.push("");
    if (val == "Hebrew") {        
        monthArray.push("Tishrei", "Chesvan", "Kislev", "Tevet", "Shvat", "Adar I", "Adar / Adar II", "Nisan", "Iyyar", "Sivan", "Tammuz", "Av", "Elul");
    } else if (val == "Julian") {
        monthArray.push("January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December");
    } else {
        monthArray.push("January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December");
    }



    for (let monthIndex in monthArray) {
        var monthName = monthArray[monthIndex];
        var monthOption = document.createElement("option");
        monthOption.name = monthName;
        monthOption.value = monthName;
        monthOption.appendChild(document.createTextNode(monthName));
        //month.appendChild(monthName);
        month.options.add(monthOption);
    }
    form.append(month);

    var dayLabel = document.createElement("label");
    dayLabel.for = "day";
    dayLabel.appendChild(document.createTextNode("Day: "));
    form.append(dayLabel);
    
    var day = document.createElement("input");
    day.type = "number";
    day.min = "1";
    day.id = "day";
    day.name = "day";
    form.appendChild(day);

    const body = document.getElementById("convertDate");
    body.append(form);

    if (document.getElementById("convertSubmit") == null){
        var submit = document.createElement("button");
        submit.id = "convertSubmit";
        submit.appendChild(document.createTextNode("Convert"));
        document.getElementById("dateDiv").appendChild(submit);
    }
    document.getElementById("convertSubmit").addEventListener("click", displayDate);

}
