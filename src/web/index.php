<?php
function cls(){
@unlink('/home/pi/tmp/go');
@unlink('/home/pi/tmp/back');
@unlink('/home/pi/tmp/right');
@unlink('/home/pi/tmp/left');
stop();
}

switch( $_REQUEST['dir'] ){
  case 'go':
  case 'back':
  case 'right':
  case 'left':
    cls();
    set_speed();
    break;
  case 'stop':
    cls();
    break;
}
require_once('controller.html');


function set_speed(){
    $x = isset($_REQUEST['R'])? $_REQUEST['R']: 50;
    $y = isset($_REQUEST['L'])? $_REQUEST['L']: 50;
    $sock = socket_create(AF_INET, SOCK_DGRAM, SOL_UDP);

    $ary = array("stop"=>0,"R"=>intval($x),"L"=>intval($y)*-1);
    $msg = json_encode($ary);
    $len = strlen($msg);

    socket_sendto($sock, $msg, $len, 0, '127.0.0.1', 5000);
    socket_close($sock);
}
function stop(){
    $sock = socket_create(AF_INET, SOCK_DGRAM, SOL_UDP);

    $ary = array("stop"=>1);
    $msg = json_encode($ary);
    $len = strlen($msg);

    socket_sendto($sock, $msg, $len, 0, '127.0.0.1', 5000);
    socket_close($sock);
}
?>