package main

import (
	"encoding/json"
	"testing"

	v1 "k8s.io/api/core/v1"
)

func BenchmarkEncode(b *testing.B) {
	pod := v1.Pod{
		Spec: v1.PodSpec{
			Containers: []v1.Container{
				{
					Name:  "example",
					Image: "example/image",
				},
			},
		},
	}
	for i := 0; i < b.N; i++ {
		_, err := json.Marshal(pod)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkDecode(b *testing.B) {
	pod := v1.Pod{
		Spec: v1.PodSpec{
			Containers: []v1.Container{
				{
					Name:  "example",
					Image: "example/image",
				},
			},
		},
	}
	data, err := json.Marshal(pod)
	if err != nil {
		b.Fatal(err)
	}
	for i := 0; i < b.N; i++ {
		var decodedPod v1.Pod
		err := json.Unmarshal(data, &decodedPod)
		if err != nil {
			b.Fatal(err)
		}
	}
}
