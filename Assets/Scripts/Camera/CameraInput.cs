using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using static Easing;

[RequireComponent(typeof(CameraControl))]
public class CameraInput : MonoBehaviour
{
    private CameraControl control;
    private Camera camera;

    public float PanSpeed = 800.0f;
    public float PanDrag  = 15.0f;
    public float MousePanSpeed = 30.0f;
    public float MousePanDrag = 0.8f;
    
    public float ZoomSpeed = 0.15f;
    public float ZoomDrag = 10.0f;

    public float RotationSpeed = 0.23f;
    
    public float MaxZoomSpeed = 2.0f;
    public float MaxPanSpeed = 1000.0f;
    public float MaxRotationSpeed = 270.0f;

    private float zoomVelocity = 0.0f;
    private Vector2 panVelocity = new Vector2(0.0f, 0.0f);
    
    private bool dragMouseButtonDown = false;
    private Vector2 dragMouseDownPos;

    private bool rotateMouseButtonDown = false;
    private Vector3 rotateMouseDownPosition;

    private Coroutine activeRotateCoroutine;
    private float rotateCoroutineTarget;

    void Awake()
    {
        control = GetComponent<CameraControl>();
        camera = GetComponent<Camera>();
    }

    void Update()
    {
        UpdateFromInput();
        DragWithMouse();
        RotateWithMouse();
    }

    void UpdateFromInput()
    {
        float zoom = Input.GetAxis("Zoom");
        zoomVelocity = Mathf.Min(MaxZoomSpeed, zoomVelocity + zoom * ZoomSpeed);
        control.Zoom = Mathf.Clamp01(control.Zoom + zoomVelocity);
        zoomVelocity = zoomVelocity * Mathf.Max(0, 1.0f - ZoomDrag*Time.deltaTime);

        // Enable rotation using rotation keys but only in angles of 90
        float rotationAngle = 0.0f;
        if(Input.GetButtonDown("RotateLeft"))
            rotationAngle = 90;
        if(Input.GetButtonDown("RotateRight"))
            rotationAngle = -90;

        if(rotationAngle > 0.0f || rotationAngle < 0.0f)
        {
            // Ensure that there is not already an animation running
            if(activeRotateCoroutine != null)
            {
                StopCoroutine(activeRotateCoroutine);
                rotateCoroutineTarget += rotationAngle;
            }
            else
                rotateCoroutineTarget = control.Rotation + rotationAngle;

            activeRotateCoroutine = StartCoroutine("AnimateRotate", rotateCoroutineTarget);
        }
        
        // Do movement with the keyboard only if not currently dragging with the mouse
        if(!dragMouseButtonDown)
        {
            Vector2 pan = new Vector2(
                Input.GetAxis("Horizontal"),
                Input.GetAxis("Vertical")) * PanSpeed * Time.deltaTime;
            Vector3 pan3d = Quaternion.Euler(0, control.Rotation, 0) * new Vector3(pan.x, 0.0f, pan.y);
            panVelocity = Vector2.ClampMagnitude(panVelocity + new Vector2(pan3d.x, pan3d.z), MaxPanSpeed);
            control.Target.transform.position += new Vector3(panVelocity.x * Time.deltaTime, 0, panVelocity.y * Time.deltaTime);
            panVelocity = panVelocity * Mathf.Max(0.0f, 1.0f - PanDrag*Time.deltaTime);
        }        
    }

    /// <summary>
    /// Enables rotating with the middle mouse button
    /// </summary>
    void RotateWithMouse()
    {
        if (Input.GetMouseButtonDown(2))
        {
            rotateMouseButtonDown = true;
            rotateMouseDownPosition = Input.mousePosition;
        }
        else if (Input.GetMouseButtonUp(2))
        {
            rotateMouseButtonDown = false;
        }
        else if (rotateMouseButtonDown)
        {
            control.Rotation += (rotateMouseDownPosition.x - Input.mousePosition.x) * RotationSpeed;
            rotateMouseDownPosition = Input.mousePosition;
        }
    }

    /// <summary>
    /// Enables dragging with the right mouse
    /// </summary>
    void DragWithMouse()
    {
        Ray ray = camera.ScreenPointToRay(Input.mousePosition);
        float distance = ray.origin.y/ray.direction.y;
        Vector2 groundPos = new Vector2(
            ray.origin.x - ray.direction.x*distance,
            ray.origin.z - ray.direction.z*distance);

        if (Input.GetMouseButtonDown(1))
        {
            dragMouseButtonDown = true;
            dragMouseDownPos = groundPos;
            
        }
        else if (Input.GetMouseButtonUp(1))
        {
            dragMouseButtonDown = false;
        }
        else if (dragMouseButtonDown)
        {
            Vector2 delta = groundPos - dragMouseDownPos;
            control.Target.transform.position -= new Vector3(delta.x, 0.0f, delta.y);
            panVelocity = panVelocity*MousePanDrag + (-delta*MousePanSpeed) * (1.0f-MousePanDrag);
        }
    }

    /// <summary>
    /// Does an interpolation from the current rotation angle to the target angle
    /// </summary>
    /// <param name="targetAngle"></param>
    /// <returns></returns>
    IEnumerator AnimateRotate(float targetAngle)
    {
        float duration = 0.3f;
        float elapsed = 0.0f;
        float startAngle = control.Rotation;
        while(elapsed < duration)
        {
            elapsed += Time.deltaTime;
            float t = Cubic.Out(Mathf.Clamp01(elapsed/duration));
            control.Rotation = Mathf.Lerp(startAngle, targetAngle, t);
            yield return null;
        }

        activeRotateCoroutine = null;
    }
}
