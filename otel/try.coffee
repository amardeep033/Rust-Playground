status API --------->     https://vpce-058cfe18eb8e45efb-255bfuxn.execute-api.ap-south-1.vpce.amazonaws.com/SIT/status?dns=EXEC_SQL_1.orch_service%22

let url = format!("{}/status?dns={}", configuration_parameters.worker_url());
 
    if not dns_param:
        return {
            "statusCode": 400,
            "body": json.dumps({"message": "dns parameter is required. Example: ?dns=gmail"})
        }
 
        return {
            "statusCode": 200,
            "body": json.dumps({
                "message": f"Found record for DNS: {dns_param}",
                "data": item
            })
        }

-----------------------------------------------------------------------------------------------------
 
create api
 
    return {"statusCode": 400, "body": json.dumps({"message": error_msg})}
 
                "statusCode": 200,
                "body": json.dumps({
                    "message": "ECS service creation started",
                    "dns": dns,
                    "executor_id": executor_id,
                    "status": "creating",
                    "mode": mode
                })

-----------------------------------------------------------------------------------------------------
 
delete api
 
                "statusCode": 400,
                "body": json.dumps({
                    "message": f"Service is {status}, can't delete",
                    "dns": dns,
                    "executor_id": executor_id,
                    "status": status
                })
  
            return {"statusCode": 400, "body": json.dumps({"message": error_msg})}
 
 
            "statusCode": 200,
            "body": json.dumps({
                "message": f"Service scaledown started for '{service_name}'",
                "dns": dns,
                "executor_id": executor_id,
                "status": "deleting"
            })
