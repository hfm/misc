package main

import (
	"context"
	"fmt"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
)

func main() {
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		panic("configuration error, " + err.Error())
	}

	client := dynamodb.NewFromConfig(cfg)

	input := &dynamodb.ListTablesInput{
		Limit: aws.Int32(5),
	}

	result, err := client.ListTables(context.TODO(), input)
	if err != nil {
		panic("configuration error, " + err.Error())
	}

	for _, table := range result.TableNames {
		fmt.Println(table)
	}
}
