
pub static EXAMPLE1: &'static str = r#"
{
    "apiVersion": "v1",
    "kind": "Endpoints",
    "metadata": {
        "annotations": {
            "endpoints.kubernetes.io/last-change-trigger-time": "2020-06-23T03:12:36Z"
        },
        "creationTimestamp": "2020-06-23T03:12:36Z",
        "labels": {
            "app": "cert-manager",
            "app.kubernetes.io/component": "controller",
            "app.kubernetes.io/instance": "cert-manager",
            "app.kubernetes.io/managed-by": "Helm",
            "app.kubernetes.io/name": "cert-manager",
            "helm.sh/chart": "cert-manager-v0.15.1"
        },
        "managedFields": [
            {
                "apiVersion": "v1",
                "fieldsType": "FieldsV1",
                "fieldsV1": {
                    "f:metadata": {
                        "f:annotations": {
                            ".": {},
                            "f:endpoints.kubernetes.io/last-change-trigger-time": {}
                        },
                        "f:labels": {
                            ".": {},
                            "f:app": {},
                            "f:app.kubernetes.io/component": {},
                            "f:app.kubernetes.io/instance": {},
                            "f:app.kubernetes.io/managed-by": {},
                            "f:app.kubernetes.io/name": {},
                            "f:helm.sh/chart": {}
                        }
                    },
                    "f:subsets": {}
                },
                "manager": "kube-controller-manager",
                "operation": "Update",
                "time": "2020-10-06T21:29:52Z"
            }
        ],
        "name": "cert-manager",
        "namespace": "cert-manager",
        "resourceVersion": "54206665",
        "selfLink": "/api/v1/namespaces/cert-manager/endpoints/cert-manager",
        "uid": "20bafbf7-9336-4ade-8e15-781fdac54f5b"
    },
    "subsets": [
        {
            "addresses": [
                {
                    "ip": "100.10.100.1",
                    "nodeName": "k8s-node-1.local",
                    "targetRef": {
                        "kind": "Pod",
                        "name": "cert-manager-5c8c96bd48-q9fp6",
                        "namespace": "cert-manager",
                        "resourceVersion": "54206534",
                        "uid": "bf497218-002e-498d-9cbe-a008d5c63c01"
                    }
                }
            ],
            "ports": [
                {
                    "port": 9402,
                    "protocol": "TCP"
                }
            ]
        }
    ]
}
"#;


pub static INGRESS : &'static str = r#"
{
    "apiVersion": "networking.k8s.io/v1beta1",
    "kind": "Ingress",
    "metadata": {
        "annotations": {
            "cert-manager.io/cluster-issuer": "letsencrypt-cf",
            "kubectl.kubernetes.io/last-applied-configuration": "{\"apiVersion\":\"extensions/v1beta1\",\"kind\":\"Ingress\",\"metadata\":{\"annotations\":{\"cert-manager.io/cluster-issuer\":\"letsencrypt-cf\",\"kubernetes.io/ingress.class\":\"kong-internal\",\"kubernetes.io/tls-acme\":\"true\"},\"name\":\"grafana\",\"namespace\":\"grafana\"},\"spec\":{\"rules\":[{\"host\":\"grafana.test.me.local\",\"http\":{\"paths\":[{\"backend\":{\"serviceName\":\"grafana\",\"servicePort\":3000}}]}}],\"tls\":[{\"hosts\":[\"grafana.test.me.local\"],\"secretName\":\"grafana-tls\"}]}}\n",
            "kubernetes.io/ingress.class": "kong-internal",
            "kubernetes.io/tls-acme": "true"
        },
        "creationTimestamp": "2020-08-09T17:32:26Z",
        "generation": 1,
        "managedFields": [
            {
                "apiVersion": "extensions/v1beta1",
                "fieldsType": "FieldsV1",
                "fieldsV1": {
                    "f:metadata": {
                        "f:annotations": {
                            ".": {},
                            "f:cert-manager.io/cluster-issuer": {},
                            "f:kubectl.kubernetes.io/last-applied-configuration": {},
                            "f:kubernetes.io/ingress.class": {},
                            "f:kubernetes.io/tls-acme": {}
                        }
                    },
                    "f:spec": {
                        "f:rules": {},
                        "f:tls": {}
                    }
                },
                "manager": "kubectl",
                "operation": "Update",
                "time": "2020-08-09T17:32:26Z"
            },
            {
                "apiVersion": "networking.k8s.io/v1beta1",
                "fieldsType": "FieldsV1",
                "fieldsV1": {
                    "f:status": {
                        "f:loadBalancer": {
                            "f:ingress": {}
                        }
                    }
                },
                "manager": "kong-ingress-controller",
                "operation": "Update",
                "time": "2020-08-09T17:32:55Z"
            }
        ],
        "name": "grafana",
        "namespace": "grafana",
        "resourceVersion": "24602774",
        "selfLink": "/apis/extensions/v1beta1/namespaces/grafana/ingresses/grafana",
        "uid": "e19ed3c2-004f-4342-9818-f55ea5526dec"
    },
    "spec": {
        "rules": [
            {
                "host": "grafana.test.me.local",
                "http": {
                    "paths": [
                        {
                            "backend": {
                                "serviceName": "grafana",
                                "servicePort": 3000
                            },
                            "pathType": "ImplementationSpecific"
                        }
                    ]
                }
            }
        ],
        "tls": [
            {
                "hosts": [
                    "grafana.test.me.local"
                ],
                "secretName": "grafana-tls"
            }
        ]
    },
    "status": {
        "loadBalancer": {
            "ingress": [
                {
                    "hostname": "internal-aa1a0c0cb6679457bbe3f1c4a5e8f748-1899515855.us-east-1.elb.amazonaws.com"
                }
            ]
        }
    }
}
"#;

