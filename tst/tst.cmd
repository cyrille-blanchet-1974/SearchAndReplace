@echo off
cd ..
cargo build
copy .\target\debug\search_and_replace.exe tst
cd tst
if exist temp rd /q/s temp
md temp
copy config.xml temp

search_and_replace /search:@@defaultEngine@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@cleaningEngine@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@updateLogosEngine@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_rmi_port@@ /replace:20081 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_mail_host@@ /replace:smtp-dc1.ehc.adp.com /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_mail_to@@ /replace:ASP.monitoring@fr.adp.com;jeannot.andriamp@fr.adp.com;stephane.manon@fr.adp.com /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_mail_from@@ /replace:ASP.monitoring@fr.adp.com /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_mail_subject@@ /replace:"Gex (Erreur {0}) : {1} !" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_mail_onlyOnError@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_http_port@@ /replace:20080 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_config_css@@ /replace:"/styles/default.css" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_log_dir@@ /replace:"/ehc/fs1/adp/sgx/logs" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_conf_dir@@ /replace:"/ehc/fs1/adp/sgx/conf" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_java_home@@ /replace:"/ehc/fs1/softs/jdk/jdk-1.8.0.291" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_unit@@ /replace:day /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_period@@ /replace:1 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_purge_time@@ /replace:"02:00:00" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_valid@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_maxRetry@@ /replace:0 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_retryAllowed@@ /replace:false  /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_secondsBeforeRetry@@ /replace:300  /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_cleaning_allowManualTrigger@@ /replace:true  /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_purge_ageinday@@ /replace:20 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_unit@@ /replace:day /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_period@@ /replace:1 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_update_logos_time@@ /replace:"02:00:00" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_valid@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_maxRetry@@ /replace:0 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_retryAllowed@@ /replace:false /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_secondsBeforeRetry@@ /replace:300 /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_logos_allowManualTrigger@@ /replace:true /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_gestil_logos_dir@@ /replace:"/ehc/fs1/adp/gtaweb/web/logo" /fic:temp\config.xml
pause
search_and_replace /search:@@sgx_service_logos_url@@ /replace:http://lx6029.ad.esi.adp.com:14085/portal-services/clientdata/logo /fic:temp\config.xml
pause
search_and_replace /search:@@install_dir@@ /replace:"/ehc/fs1/adp/sgx" /fic:temp\config.xml
pause
search_and_replace /search:@@oracle_home@@ /replace:"/ehc/fs1/softs/app/oracle/product/11.2.0" /fic:temp\config.xml
pause