/* The MIT License (MIT)
 *
 * Copyright (c) 2018 Nathan Osman
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

$(function() {

  var TIME_FMT = "h:mm A";

  // Grab a reference to the location fields and calculate button
  var $latitude = $('#latitude input');
  var $longitude = $('#longitude input');

  // Initialize the date field
  var $date = $('#date')
  .calendar({
    type: 'date'
  })
  .calendar('set date', new Date());

  function calculate() {
    $('#result').hide();
    $('.ui.dimmer').addClass('active');
    var d = $date.calendar('get date');
    $.ajax({
      method: 'POST',
      url: "/api",
      contentType: 'application/json',
      data: JSON.stringify({
        latitude: parseFloat($latitude.val()),
        longitude: parseFloat($longitude.val()),
        year: 1900 + d.getYear(),
        month: 1 + d.getMonth(),
        day: d.getDate()
      })
    })
    .done(function(d) {
      $('#sunrise').text(moment.unix(d.sunrise).format(TIME_FMT));
      $('#sunset').text(moment.unix(d.sunset).format(TIME_FMT));
      $('#result').show();
    })
    .always(function() {
      $('.ui.dimmer').removeClass('active');
    });
  }

  // If location information is available, use it
  if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(function(pos) {
      $latitude.val(pos.coords.latitude);
      $longitude.val(pos.coords.longitude);
      calculate();
    });
  }

  // Perform calculation when the button is clicked
  $('#calculate').click(calculate);

});
