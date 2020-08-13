<?php
    $sock = socket_create(AF_INET, SOCK_DGRAM, SOL_UDP);

    $ary = array("stop"=>1,"R"=>100,"L"=>0);
    $msg = json_encode($ary);
    $len = strlen($msg);

    socket_sendto($sock, $msg, $len, 0, '127.0.0.1', 5000);
    socket_close($sock);
?>