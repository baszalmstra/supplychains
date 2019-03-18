using System.Collections;
using System.Collections.Generic;
using UnityEngine;

[ExecuteInEditMode]
public class CameraControl : MonoBehaviour
{
    public GameObject Target;
    
    public float MinDistance = 10.0f;
    public float MaxDistance = 60.0f;
    public float MinAngle = 40.0f;
    public float MaxAngle = 60.0f;
    [RangeAttribute(0.0f, 1.0f)]
    public float Zoom = 0.5f;
    public float Rotation = 30.0f;

    public float CurrentDistance 
    {
        get 
        { 
            return MinDistance + (MaxDistance - MinDistance) * Zoom; 
        }
    }

    void Update()
    {
        Apply();
    }

    public void Apply()
    {
        float angle = Mathf.Lerp(MinAngle, MaxAngle, Zoom);

        Quaternion orientation = Quaternion.Euler(angle, Rotation, 0);
        transform.rotation = orientation;
        transform.position = orientation * new Vector3(0,0,-CurrentDistance) + Target.transform.position;
    }
}
