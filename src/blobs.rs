
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

